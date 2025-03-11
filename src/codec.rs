use crate::CodecError;

use serde::{Deserialize, Serialize};

type Result<T, E = CodecError> = std::result::Result<T, E>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Codec {
    level: i32,
}

impl Codec {
    /// Creates a new Codec struct.
    ///
    /// # Arguments
    ///
    /// * `level` - The level of compression to use. 0 is no compression, 1 is fastest, 22 is slowest.
    ///   Check the [zstd documentation](https://github.com/facebook/zstd) for more information.
    ///
    /// # Panics
    ///
    /// This function will panic if the compression level is outside the range 0-22.
    #[must_use]
    pub fn new(level: i32) -> Self {
        assert!(level <= 22, "level should be >= 0 and <= 22");
        Self { level }
    }

    /// Serializes and compresses the provided data using the `MessagePack` format.
    /// This will reduce the size of the data and make it easier to compress.
    /// From testing I found that a level of 1 was a good balance between compression and size.
    /// The average reduction is around 85% of the original, whilst being slightly faster to compress
    /// and decompress.
    ///
    /// # Errors
    ///
    /// Return `epoch_archive::CodecError` if there is an issue serializing or compressing the data.
    pub fn encode<T: Serialize>(&self, data: &T) -> Result<Vec<u8>> {
        let serialized = Self::serialize(data)?;
        self.compress(&serialized)
    }

    /// Deserializes and decompresses the provided data using the `MessagePack` format.
    ///
    /// # Errors
    ///
    /// Return `epoch_archive::CodecError` if there is an issue deserializing or decompressing the data.
    pub fn decode<T>(&self, data: &[u8]) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let decompressed = self.decompress(data)?;
        let deserialized = self.deserialize::<T>(&decompressed)?;

        Ok(deserialized)
    }

    /// Compresses the provided data using the zstd algorithm.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to be compressed.
    ///
    /// # Errors
    ///
    /// Return `epoch_archive::CodecError` if there is an issue compressing the data.
    pub fn compress(&self, data: &[u8]) -> Result<Vec<u8>> {
        Ok(zstd::encode_all(data, self.level)?)
    }

    /// Decompresses the provided data using the zstd algorithm.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to be decompressed.
    ///
    /// # Errors
    ///
    /// Return `epoch_archive::CodecError` if there is an issue decompressing the data.
    pub fn decompress(&self, data: &[u8]) -> Result<Vec<u8>> {
        Ok(zstd::decode_all(data)?)
    }

    /// Serializes the provided data using the `MessagePack` format.
    ///
    /// # Errors
    ///
    /// Return `epoch_archive::CodecError` if there is an issue serializing the data.
    pub fn serialize<T: Serialize>(data: &T) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        let mut ser = rmp_serde::Serializer::new(&mut buf);
        data.serialize(&mut ser)?;

        Ok(buf)
    }

    /// Deserializes the provided data using the `MessagePack` format.
    ///
    /// # Errors
    ///
    /// Return `rmp_serde::decode::Error` if there is an issue deserializing the data.
    pub fn deserialize<'a, T>(&self, data: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a>,
    {
        Ok(rmp_serde::from_slice(data)?)
    }
}

impl Default for Codec {
    fn default() -> Self {
        Self { level: 1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let codec = Codec::new(3);
        assert_eq!(codec.level, 3);
    }

    #[test]
    fn test_default() {
        let codec = Codec::default();
        assert_eq!(codec.level, 1);
    }

    #[test]
    #[should_panic(expected = "level should be >= 0 and <= 22")]
    fn test_new_too_high_level() {
        #[allow(unused_must_use)]
        Codec::new(23);
    }

    #[test]
    fn test_compress() {
        let data = vec![1, 2, 3, 4, 5];

        for i in 0..22 {
            let codec = Codec::new(i);
            let compressed = codec.compress(&data).unwrap();
            assert_ne!(data, compressed);
        }
    }

    #[test]
    fn test_decompress() {
        let expected = vec![1, 2, 3, 4, 5];
        let compressed = [40, 181, 47, 253, 0, 72, 41, 0, 0, 1, 2, 3, 4, 5];
        let codec = Codec::new(1);

        let decompressed = codec.decompress(&compressed).unwrap();
        assert_eq!(decompressed, expected);
    }

    #[test]
    fn test_decompress_fail_invalid_data() {
        let invalid: [u8; 14] = [
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        ];
        let codec = Codec::new(1);

        let decompressed = codec.decompress(&invalid);
        assert!(decompressed.is_err());
    }

    #[test]
    fn test_encode() {
        let data = vec![1, 2, 3, 4, 5];
        let codec = Codec::new(1);

        let encoded = codec.encode(&data).unwrap();
        let expected = [40, 181, 47, 253, 0, 72, 49, 0, 0, 149, 1, 2, 3, 4, 5];
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_decode() {
        let encoded = [40, 181, 47, 253, 0, 72, 49, 0, 0, 149, 1, 2, 3, 4, 5];
        let expected = vec![1, 2, 3, 4, 5];
        let codec = Codec::new(1);

        let decoded = codec.decode::<Vec<u8>>(&encoded).unwrap();
        assert_eq!(decoded, expected);
    }
}
