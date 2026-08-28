use hashcheck::{hash_file, normalize_expected, Algorithm};
use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::ExitCode;

const EXIT_MISMATCH: u8 = 1;
const EXIT_USAGE_OR_IO: u8 = 2;

fn main() -> ExitCode {
    match run(env::args_os().skip(1)) {
        Ok(code) => ExitCode::from(code),
        Err(message) => {
            eprintln!("hashcheck: {message}");
            ExitCode::from(EXIT_USAGE_OR_IO)
        }
    }
}

fn run<I>(args: I) -> Result<u8, String>
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

fn print_help() {
    println!(
        "HashCheck {}\n\
Simple SHA-256/SHA-512 checksum verification.\n\n\
USAGE:\n\
    hashcheck [--sha256 | --sha512] <FILE> [--expect <HASH>]\n\n\
OPTIONS:\n\
    --sha256          Use SHA-256 (default)\n\
    --sha512          Use SHA-512\n\
    --expect <HASH>   Verify against an expected hexadecimal checksum\n\
    -h, --help        Print help\n\
    -V, --version     Print version\n\n\
EXIT CODES:\n\
    0  Hash calculated successfully or expected checksum matched\n\
    1  Expected checksum did not match\n\
    2  Usage or I/O error",
        env!("CARGO_PKG_VERSION")
    );
}
