pub mod structs {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
    pub struct Simple {
        pub numbers: Vec<u32>,
        pub letters: Vec<char>,
    }

    impl Default for Simple {
        fn default() -> Self {
            Self {
                numbers: vec![1, 2, 3, 4, 5],
                letters: vec!['a', 'b', 'c', 'd', 'e'],
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
    pub struct Complex {
        pub strings: Strings,
        pub number: u64,
        pub simple: Simple,
        pub simples: Vec<Simple>,
    }

    impl Default for Complex {
        fn default() -> Self {
            Self {
                strings: Strings::default(),
                number: 42,
                simple: Simple::default(),
                simples: (0..100).map(|_| Simple::default()).collect(),
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
    pub struct Strings {
        foo: String,
        bar: String,
        baz: String,
        qux: String,
        quux: String,
    }

    impl Default for Strings {
        fn default() -> Self {
            Self {
                foo: String::from("foo-string"),
                bar: String::from("bar-string"),
                baz: String::from("baz-string"),
                qux: String::from("qux-string"),
                quux: String::from("quux-string"),
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
    #[serde(untagged)]
    pub enum SimpleOrComplex {
        Simple(Simple),
        Complex(Complex),
    }
}
