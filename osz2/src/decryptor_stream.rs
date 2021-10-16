use std::io::Read;

pub struct DecryptorStream<T> {
    key: [u32; 4],
    pub inner: T,
}

impl<T> DecryptorStream<T> {
    pub fn new(inner: T, key: [u32; 4]) -> Self {
        DecryptorStream { key, inner }
    }
}

impl<T: Read> Read for DecryptorStream<T> {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        let count = self.inner.read(buffer)?;
        crate::xxtea::decrypt(&self.key, buffer);
        Ok(count)
    }
}
