use sha2::{Digest, Sha256, Sha512};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

const BUFFER_SIZE: usize = 64 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Algorithm {
    Sha256,
    Sha512,
}

impl Algorithm {
    pub const fn expected_hex_len(self) -> usize {
        match self {
            Self::Sha256 => 64,
            Self::Sha512 => 128,
        }
    }
}

pub fn hash_file(path: &Path, algorithm: Algorithm) -> io::Result<String> {
    let file = File::open(path)?;
    hash_reader(file, algorithm)
}

pub fn hash_reader<R: Read>(mut reader: R, algorithm: Algorithm) -> io::Result<String> {
    match algorithm {
        Algorithm::Sha256 => digest_reader::<Sha256, _>(&mut reader),
        Algorithm::Sha512 => digest_reader::<Sha512, _>(&mut reader),
    }
}

pub fn normalize_expected(expected: &str, algorithm: Algorithm) -> Result<String, &'static str> {
    if expected.len() != algorithm.expected_hex_len() {
        return Err("checksum has the wrong length for the selected algorithm");
    }

    if !expected.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err("checksum must contain only hexadecimal characters");
    }

    Ok(expected.to_ascii_lowercase())
}

fn digest_reader<D, R>(reader: &mut R) -> io::Result<String>
where
    D: Digest + Default,
    R: Read,
{
    let mut hasher = D::new();
    let mut buffer = [0_u8; BUFFER_SIZE];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let digest = hasher.finalize();
    Ok(to_lower_hex(digest.as_ref()))
}

fn to_lower_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);

    for &byte in bytes {
        output.push(char::from(HEX[(byte >> 4) as usize]));
        output.push(char::from(HEX[(byte & 0x0f) as usize]));
    }

    output
}

#[cfg(test)]
mod tests {
    use super::{hash_reader, normalize_expected, Algorithm};
    use std::io::Cursor;

    #[test]
    fn sha256_known_vector() {
        let digest = hash_reader(Cursor::new(b"abc"), Algorithm::Sha256).unwrap();
        assert_eq!(
            digest,
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn sha512_known_vector() {
        let digest = hash_reader(Cursor::new(b"abc"), Algorithm::Sha512).unwrap();
        assert_eq!(
            digest,
            "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"
        );
    }

    #[test]
    fn expected_checksum_is_case_insensitive() {
        let normalized = normalize_expected(
            "BA7816BF8F01CFEA414140DE5DAE2223B00361A396177A9CB410FF61F20015AD",
            Algorithm::Sha256,
        )
        .unwrap();

        assert_eq!(
            normalized,
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn expected_checksum_rejects_non_hex_input() {
        assert!(normalize_expected(&"z".repeat(64), Algorithm::Sha256).is_err());
    }
}
