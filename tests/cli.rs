use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn temp_file(contents: &[u8]) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "hashcheck-test-{}-{id}.bin",
        std::process::id()
    ));
    fs::write(&path, contents).unwrap();
    path
}

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_hashcheck"))
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn defaults_to_sha256() {
    let path = temp_file(b"abc");
    let output = run(&[path.to_str().unwrap()]);
    let _ = fs::remove_file(path);

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad\n"
    );
}

#[test]
fn supports_sha512() {
    let path = temp_file(b"abc");
    let output = run(&["--sha512", path.to_str().unwrap()]);
    let _ = fs::remove_file(path);

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f\n"
    );
}

#[test]
fn accepts_uppercase_expected_checksum() {
    let path = temp_file(b"abc");
    let output = run(&[
        path.to_str().unwrap(),
        "--expect",
        "BA7816BF8F01CFEA414140DE5DAE2223B00361A396177A9CB410FF61F20015AD",
    ]);
    let _ = fs::remove_file(path);

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "OK\n");
}

#[test]
fn mismatch_exits_with_one() {
    let path = temp_file(b"abc");
    let output = run(&[
        path.to_str().unwrap(),
        "--expect",
        "0000000000000000000000000000000000000000000000000000000000000000",
    ]);
    let _ = fs::remove_file(path);

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "MISMATCH\n");
}

#[test]
fn invalid_expected_checksum_exits_with_two() {
    let path = temp_file(b"abc");
    let output = run(&[path.to_str().unwrap(), "--expect", "not-a-checksum"]);
    let _ = fs::remove_file(path);

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
}

#[test]
fn missing_file_exits_with_two() {
    let path = std::env::temp_dir().join(format!(
        "hashcheck-definitely-missing-{}.bin",
        std::process::id()
    ));
    let output = run(&[path.to_str().unwrap()]);

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
}
