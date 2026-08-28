use hashcheck::{hash_file, normalize_expected, Algorithm};
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::{self, ExitCode};

const EXIT_MISMATCH: u8 = 1;
const EXIT_USAGE_OR_IO: u8 = 2;

fn main() -> ExitCode {
    match run(env::args_os().skip(1).collect()) {
        Ok(code) => ExitCode::from(code),
        Err(message) => {
            eprintln!("hashcheck: {message}");
            ExitCode::from(EXIT_USAGE_OR_IO)
        }
    }
}

fn run(args: Vec<OsString>) -> Result<u8, String> {
    if args.first().and_then(|arg| arg.to_str()) == Some("manifest") {
        return run_manifest(&args[1..]);
    }

    run_single_file(args)
}

fn run_single_file<I>(args: I) -> Result<u8, String>
where
    I: IntoIterator<Item = OsString>,
{
    let mut algorithm = Algorithm::Sha256;
    let mut expected: Option<String> = None;
    let mut file: Option<PathBuf> = None;
    let mut options_enabled = true;
    let mut iter = args.into_iter();

    while let Some(arg) = iter.next() {
        if options_enabled {
            match arg.to_str() {
                Some("--") => {
                    options_enabled = false;
                    continue;
                }
                Some("--sha512") => {
                    algorithm = Algorithm::Sha512;
                    continue;
                }
                Some("--sha256") => {
                    algorithm = Algorithm::Sha256;
                    continue;
                }
                Some("--expect") => {
                    let value = iter
                        .next()
                        .ok_or_else(|| "--expect requires a checksum value".to_owned())?;
                    let value = value
                        .into_string()
                        .map_err(|_| "checksum must be valid UTF-8 hexadecimal text".to_owned())?;
                    if expected.replace(value).is_some() {
                        return Err("--expect may only be specified once".to_owned());
                    }
                    continue;
                }
                Some("-h" | "--help") => {
                    print_help();
                    return Ok(0);
                }
                Some("-V" | "--version") => {
                    println!("hashcheck {}", env!("CARGO_PKG_VERSION"));
                    return Ok(0);
                }
                Some(value) if value.starts_with('-') => {
                    return Err(format!("unknown option: {value}"));
                }
                _ => {}
            }
        }

        if file.replace(PathBuf::from(arg)).is_some() {
            return Err("exactly one file path must be provided".to_owned());
        }
    }

    let file = file.ok_or_else(|| "missing file path; try --help".to_owned())?;

    let expected = expected
        .as_deref()
        .map(|value| normalize_expected(value, algorithm).map_err(str::to_owned))
        .transpose()?;

    let actual = hash_file(&file, algorithm)
        .map_err(|error| format!("failed to read '{}': {error}", file.display()))?;

    match expected {
        Some(expected) if actual == expected => {
            println!("OK");
            Ok(0)
        }
        Some(_) => {
            println!("MISMATCH");
            Ok(EXIT_MISMATCH)
        }
        None => {
            println!("{actual}");
            Ok(0)
        }
    }
}

fn run_manifest(args: &[OsString]) -> Result<u8, String> {
    let action = args
        .first()
        .and_then(|arg| arg.to_str())
        .ok_or_else(|| "manifest requires 'create' or 'verify'; try --help".to_owned())?;

    match action {
        "create" if args.len() >= 3 => {
            let manifest = PathBuf::from(&args[1]);
            let files = args[2..].iter().map(PathBuf::from).collect::<Vec<_>>();
            create_manifest(&manifest, &files)?;
            println!("WROTE  {}", manifest.display());
            Ok(0)
        }
        "verify" if args.len() == 2 => {
            let manifest = PathBuf::from(&args[1]);
            if verify_manifest(&manifest)? {
                Ok(0)
            } else {
                Ok(EXIT_MISMATCH)
            }
        }
        "create" => Err("usage: hashcheck manifest create <MANIFEST> <FILE>...".to_owned()),
        "verify" => Err("usage: hashcheck manifest verify <MANIFEST>".to_owned()),
        _ => Err("manifest action must be 'create' or 'verify'".to_owned()),
    }
}

fn manifest_parent(manifest: &Path) -> &Path {
    manifest
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."))
}

fn manifest_name(manifest: &Path, file: &Path) -> Result<String, String> {
    let parent = manifest_parent(manifest);
    let stored = if parent == Path::new(".") {
        if file.is_absolute() {
            return Err(format!(
                "release file '{}' must be relative to the manifest directory",
                file.display()
            ));
        }
        file
    } else {
        file.strip_prefix(parent).map_err(|_| {
            format!(
                "release file '{}' must be inside manifest directory '{}'",
                file.display(),
                parent.display()
            )
        })?
    };

    let value = stored
        .to_str()
        .ok_or_else(|| format!("path '{}' is not valid UTF-8", stored.display()))?;
    validate_manifest_entry_path(value)?;
    Ok(value.to_owned())
}

fn validate_manifest_entry_path(value: &str) -> Result<(), String> {
    if value.is_empty() || value.contains(['\n', '\r']) {
        return Err("manifest entry path is empty or contains a line break".to_owned());
    }

    let path = Path::new(value);
    if path.is_absolute()
        || path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(format!(
            "manifest entry '{value}' must stay inside the manifest directory"
        ));
    }
    Ok(())
}

fn atomic_write(path: &Path, content: &str) -> Result<(), String> {
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| "manifest path has no UTF-8 file name".to_owned())?;
    let temp = path.with_file_name(format!(".{file_name}.tmp.{}", process::id()));

    fs::write(&temp, content)
        .map_err(|error| format!("failed to write '{}': {error}", temp.display()))?;
    fs::rename(&temp, path)
        .map_err(|error| format!("failed to replace '{}': {error}", path.display()))?;
    Ok(())
}

fn create_manifest(manifest: &Path, files: &[PathBuf]) -> Result<(), String> {
    if files.is_empty() {
        return Err("at least one release file is required".to_owned());
    }

    let mut entries = Vec::with_capacity(files.len());
    for file in files {
        if !file.is_file() {
            return Err(format!("'{}' is not a regular file", file.display()));
        }
        if file == manifest {
            return Err("manifest cannot include itself".to_owned());
        }
        let name = manifest_name(manifest, file)?;
        let digest = hash_file(file, Algorithm::Sha256)
            .map_err(|error| format!("failed to read '{}': {error}", file.display()))?;
        entries.push((name, digest));
    }
    entries.sort_by(|left, right| left.0.cmp(&right.0));

    let mut content = String::new();
    for (name, digest) in entries {
        content.push_str(&format!("{digest}  {name}\n"));
    }
    atomic_write(manifest, &content)
}

fn parse_manifest(input: &str) -> Result<Vec<(String, String)>, String> {
    let mut entries = Vec::new();
    for (index, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let Some((digest, path)) = line.split_once("  ") else {
            return Err(format!("invalid manifest line {}", index + 1));
        };
        if digest.len() != 64 || !digest.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(format!("invalid SHA-256 digest on line {}", index + 1));
        }
        validate_manifest_entry_path(path)
            .map_err(|error| format!("invalid path on line {}: {error}", index + 1))?;
        entries.push((digest.to_ascii_lowercase(), path.to_owned()));
    }
    if entries.is_empty() {
        return Err("manifest contains no files".to_owned());
    }
    Ok(entries)
}

fn verify_manifest(manifest: &Path) -> Result<bool, String> {
    let input = fs::read_to_string(manifest)
        .map_err(|error| format!("failed to read '{}': {error}", manifest.display()))?;
    let entries = parse_manifest(&input)?;
    let parent = manifest_parent(manifest);
    let mut all_ok = true;

    for (expected, entry) in entries {
        let path = parent.join(&entry);
        match hash_file(&path, Algorithm::Sha256) {
            Ok(actual) if actual == expected => println!("OK  {entry}"),
            Ok(_) => {
                println!("MISMATCH  {entry}");
                all_ok = false;
            }
            Err(error) => {
                println!("ERROR  {entry}: {error}");
                all_ok = false;
            }
        }
    }
    Ok(all_ok)
}

fn print_help() {
    println!(
        "HashCheck {}\n\
Checksum calculation, verification, and deterministic release manifests.\n\n\
USAGE:\n\
    hashcheck [--sha256 | --sha512] <FILE> [--expect <HASH>]\n\
    hashcheck manifest create <MANIFEST> <FILE>...\n\
    hashcheck manifest verify <MANIFEST>\n\n\
OPTIONS:\n\
    --sha256          Use SHA-256 (default)\n\
    --sha512          Use SHA-512\n\
    --expect <HASH>   Verify against an expected hexadecimal checksum\n\
    -h, --help        Print help\n\
    -V, --version     Print version\n\n\
MANIFESTS:\n\
    Release manifests use SHA-256, deterministic path ordering, and relative paths\n\
    constrained to the manifest directory.\n\n\
EXIT CODES:\n\
    0  Hash calculated, expected checksum matched, or manifest verified\n\
    1  Expected checksum or one or more manifest entries did not match\n\
    2  Usage, manifest, or I/O error",
        env!("CARGO_PKG_VERSION")
    );
}

#[cfg(test)]
mod tests {
    use super::{parse_manifest, validate_manifest_entry_path};

    #[test]
    fn parses_standard_manifest_line() {
        let input =
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad  app.tar.gz\n";
        let entries = parse_manifest(input).expect("valid manifest");
        assert_eq!(entries[0].1, "app.tar.gz");
    }

    #[test]
    fn rejects_short_manifest_digest() {
        assert!(parse_manifest("abc  app.tar.gz\n").is_err());
    }

    #[test]
    fn rejects_manifest_parent_traversal() {
        assert!(validate_manifest_entry_path("../secret.txt").is_err());
    }

    #[test]
    fn accepts_nested_relative_manifest_path() {
        assert!(validate_manifest_entry_path("linux/app.tar.gz").is_ok());
    }
}
