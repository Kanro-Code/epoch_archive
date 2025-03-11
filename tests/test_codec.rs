extern crate epoch_archive;
mod test_helpers;

use epoch_archive::Codec;

#[cfg(test)]
mod tests {
    use test_helpers::structs::{Complex, Simple, SimpleOrComplex};

    use super::*;
    #[test]
    fn test_simple_string() {
        let data = std::fs::read_to_string("./tests/data/string.txt").unwrap();

        let codec = Codec::new(1);
        let compressed = codec.encode(&data).unwrap();
        let decompressed = codec.decode::<String>(&compressed).unwrap();
        assert_eq!(data, decompressed);
        assert!(data.len() > compressed.len());
    }

    #[test]
    fn test_simple_struct() {
        let codec = Codec::new(1);
        let data = Simple::default();

        let compressed = codec.encode(&data).unwrap();
        let decompressed = codec.decode::<Simple>(&compressed).unwrap();
        assert_eq!(data, decompressed);
    }

    #[test]
    fn test_complex_struct() {
        let codec = Codec::new(1);
        let complex = Complex::default();

        let compressed = codec.encode(&complex).unwrap();
        let decompressed = codec.decode::<Complex>(&compressed).unwrap();
        assert_eq!(complex, decompressed);
    }

    #[test]
    fn test_all_levels() {
        for i in 0..22 {
            let codec = Codec::new(i);

            let compressed = codec.encode(&Simple::default()).unwrap();
            let decompressed = codec.decode::<Simple>(&compressed).unwrap();
            assert_eq!(Simple::default(), decompressed);
        }
    }

    #[test]
    fn test_serde_untagged() {
        let codec = Codec::new(1);
        let simple = Simple::default();

        let compressed = codec.encode(&simple).unwrap();
        let decompressed = codec.decode::<SimpleOrComplex>(&compressed).unwrap();

        assert!(matches!(decompressed, SimpleOrComplex::Simple(_)));
    }
}
