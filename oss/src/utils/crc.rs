/// Represents a Crc64 calculator.
#[allow(unused)]
pub(crate) struct Crc64 {
    init: u64,
    crc: u64,
    crc_instance: crc::Crc<u64>,
}

#[allow(unused)]
impl Crc64 {
    /// Creates a new `Crc64` instance.
    ///
    /// # Arguments
    ///
    /// * `init` - The initial CRC value.
    pub fn new(init: u64) -> Self {
        Crc64 {
            init,
            crc: init,
            crc_instance: crc::Crc::<u64>::new(&crc::CRC_64_ECMA_182),
        }
    }

    /// Returns the size of the CRC in bytes.
    pub fn size(&self) -> u8 {
        self.crc_instance.algorithm.width / 8
    }

    /// Returns the block size for CRC calculation.
    pub fn block_size(&self) -> usize {
        1
    }

    /// Resets the CRC to its initial value.
    pub fn reset(&mut self) {
        self.crc = self.init;
    }

    /// Writes the given buffer to the CRC calculation.
    pub fn write(&mut self, buf: &[u8]) -> Result<usize, Box<dyn std::error::Error>> {
        self.crc = self.crc_instance.checksum(buf);
        Ok(buf.len())
    }

    /// Returns the current CRC value.
    pub fn sum64(&self) -> u64 {
        self.crc
    }

    /// Appends the current CRC value to the given byte vector.
    ///
    /// # Arguments
    ///
    /// * `in_bytes` - The byte vector to append the CRC value to.
    pub fn append_sum(&self, in_bytes: &mut Vec<u8>) {
        in_bytes.extend_from_slice(&self.sum64().to_be_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &[u8; 5] = b"Hello";
    const EXAMPLE_CHECKSUM: u64 = 0xFFAD3236B47900AB;

    #[test]
    fn test_new() {
        let crc = Crc64::new(0x123);
        assert_eq!(crc.init, 0x123);
        assert_eq!(crc.crc, 0x123);
    }

    #[test]
    fn test_size() {
        let crc = Crc64::new(0);
        assert_eq!(crc.size(), 8);
    }

    #[test]
    fn test_block_size() {
        let crc = Crc64::new(0);
        assert_eq!(crc.block_size(), 1);
    }

    #[test]
    fn test_reset() {
        let mut crc = Crc64::new(0);
        crc.write(b"Anything").unwrap();
        crc.reset();
        assert_eq!(crc.crc, 0);

        let mut crc = Crc64::new(0x12);
        crc.write(b"Another anything").unwrap();
        crc.reset();
        assert_eq!(crc.crc, 0x12);
    }

    #[test]
    fn test_write() {
        let mut crc = Crc64::new(0);
        crc.write(EXAMPLE_INPUT).unwrap();
        assert_eq!(crc.crc, EXAMPLE_CHECKSUM);
    }

    #[test]
    fn test_sum64() {
        let mut crc = Crc64::new(0);
        assert_eq!(crc.sum64(), 0);

        crc.write(EXAMPLE_INPUT).unwrap();
        assert_eq!(crc.crc, EXAMPLE_CHECKSUM);
    }

    #[test]
    fn test_append_sum() {
        let crc = Crc64::new(0);
        let mut in_bytes = vec![];
        crc.append_sum(&mut in_bytes);
        assert_eq!(in_bytes, [0; 8]);

        let mut crc = Crc64::new(0);
        let mut in_bytes = EXAMPLE_INPUT.clone().to_vec();
        crc.write(EXAMPLE_INPUT).unwrap();
        crc.append_sum(&mut in_bytes);
        assert_eq!(
            in_bytes,
            [EXAMPLE_INPUT, EXAMPLE_CHECKSUM.to_be_bytes().as_ref()].concat()
        );
    }
}
