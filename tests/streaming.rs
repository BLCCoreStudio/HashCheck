use hashcheck::{hash_reader, Algorithm};
use std::io::{self, Cursor, Read};

struct Chunked<R> {
    inner: R,
    max_chunk: usize,
}

impl<R: Read> Read for Chunked<R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let limit = buffer.len().min(self.max_chunk);
        self.inner.read(&mut buffer[..limit])
    }
}

#[test]
fn sha256_is_stable_across_small_reader_chunks() {
    let payload = vec![0x5a_u8; 64 * 1024 + 17];
    let expected = hash_reader(Cursor::new(&payload), Algorithm::Sha256).unwrap();
    let chunked = Chunked {
        inner: Cursor::new(&payload),
        max_chunk: 7,
    };

    let actual = hash_reader(chunked, Algorithm::Sha256).unwrap();
    assert_eq!(actual, expected);
}
