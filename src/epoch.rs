#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Epoch {
    epoch: i64,
    precision: Precision,
    delimiter: char,
}

impl Epoch {
    /// Creates a new Epoch struct.
    ///
    /// # Parameters
    ///
    /// - `epoch`: The epoch value. This is the number of seconds since the Unix epoch. If the epoch is negative, the milliseconds will be displayed as a negative number.
    ///
    /// # Examples
    ///
    /// ```
    /// use epoch_archive::Epoch;
    ///
    /// let epoch = Epoch::new(1337);
    /// assert_eq!(epoch.epoch(), 1337);
    /// ```
    pub fn new(epoch: i64) -> Self {
        Self {
            epoch,
            ..Default::default()
        }
    }

    /// Sets the epoch value.
    ///
    /// # Examples
    ///
    /// ```
    /// use epoch_archive::Epoch;
    ///
    /// let epoch = Epoch::new(1337).with_epoch(123);
    /// assert_eq!(epoch.epoch(), 123);
    /// ```
    pub fn with_epoch(self, epoch: i64) -> Self {
        Self { epoch, ..self }
    }

    /// Sets the millisecond value.
    /// If another precision is already set, this will override it.
    ///
    /// # Panics
    /// Will panic if ms is >= 1000
    ///
    /// # Examples
    ///
    /// ```
    /// use epoch_archive::{Epoch, Precision};
    ///
    /// let epoch = Epoch::new(0).with_millis(123);
    /// assert!(matches!(epoch.precision(), Precision::Milli(123)));
    /// ```
    pub fn with_millis(self, millis: u16) -> Self {
        assert!(millis < 1000, "assertion failed: millis < 1000");
        Self {
            precision: Precision::Milli(millis),
            ..self
        }
    }

    /// Sets the microsecond value.
    /// If another precision is already set, this will override it.
    ///
    /// # Panics
    /// Will panic if micros is >= 1000000
    ///
    /// # Examples
    ///
    /// ```
    /// use epoch_archive::{Epoch, Precision};
    ///
    /// let epoch = Epoch::new(0).with_micros(123);
    /// assert!(matches!(epoch.precision(), Precision::Micro(123)));
    /// ```
    pub fn with_micros(self, micros: u32) -> Self {
        assert!(micros < 1000000, "assertion failed: micros < 1000000");
        Self {
            precision: Precision::Micro(micros),
            ..self
        }
    }

    /// Sets the nanosecond value.
    /// If another precision is already set, this will override it.
    ///
    /// # Panics
    /// Will panic if ns is >= 1000000000
    ///
    /// # Examples
    ///
    /// ```
    /// use epoch_archive::{Epoch, Precision};
    ///
    /// let epoch = Epoch::new(0).with_nanos(123);
    /// assert!(matches!(epoch.precision(), Precision::Nano(123)));
    /// ```
    pub fn with_nanos(self, nanos: u64) -> Self {
        assert!(nanos < 1000000000, "assertion failed: nanos < 1000000000");
        Self {
            precision: Precision::Nano(nanos),
            ..self
        }
    }

    /// Set the delimiter character that is used to separate the epoch value from the millisecond value.
    ///
    /// # Examples
    ///
    /// ```
    /// use epoch_archive::{Epoch, Precision};
    ///
    /// let epoch = Epoch::new(1337).with_millis(123).with_delimiter(':');
    /// assert_eq!(epoch.to_string(), "1337:123");
    /// ```
    pub fn with_delimiter(self, delimiter: char) -> Self {
        Self { delimiter, ..self }
    }

    // -----------------------------
    // ---------- GETTERS ----------
    // -----------------------------

    /// Returns the epoch value.
    pub fn epoch(&self) -> i64 {
        self.epoch
    }

    /// Returns the optional millisecond value.
    ///
    /// If no value is present, this returns None.
    pub fn precision(&self) -> &Precision {
        &self.precision
    }

    /// Returns the delimiter character.
    pub fn delimiter(&self) -> char {
        self.delimiter
    }
}

impl std::fmt::Display for Epoch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.precision {
            Precision::None => write!(f, "{:}", self.epoch),
            Precision::Milli(ms) => write!(f, "{:}{}{:03}", self.epoch, self.delimiter, ms),
            Precision::Micro(us) => write!(f, "{:}{}{:06}", self.epoch, self.delimiter, us),
            Precision::Nano(ns) => write!(f, "{:}{}{:09}", self.epoch, self.delimiter, ns),
        }
    }
}

impl Default for Epoch {
    fn default() -> Self {
        Self {
            epoch: 0,
            precision: Precision::None,
            delimiter: '.',
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precision {
    None,
    Milli(u16),
    Micro(u32),
    Nano(u64),
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_EPOCH: [i64; 9] = [
        0,
        1,
        -1,
        123,
        -123,
        i64::MAX,
        i64::MIN,
        i64::MAX / 1000,
        i64::MIN / 1000,
    ];

    const TEST_MS: [u16; 4] = [0, 1, 999, 123];
    const TEST_US: [u32; 4] = [0, 1, 999999, 123123];
    const TEST_NS: [u64; 4] = [0, 1, 999999999, 123123123];

    #[test]
    fn test_new() {
        for epoch in TEST_EPOCH {
            let new = Epoch::new(epoch);
            assert_eq!(new.epoch, epoch);
        }
    }

    #[test]
    fn test_with_milli() {
        for epoch in TEST_EPOCH {
            for ms in TEST_MS {
                let new = Epoch::new(epoch).with_millis(ms);
                assert_eq!(new.precision, Precision::Milli(ms));
                assert_eq!(new.epoch, epoch);
            }
        }
    }

    #[test]
    fn test_with_micro() {
        for epoch in TEST_EPOCH {
            for ms in TEST_US {
                let new = Epoch::new(epoch).with_micros(ms);
                assert_eq!(new.precision, Precision::Micro(ms));
                assert_eq!(new.epoch, epoch);
            }
        }
    }

    #[test]
    fn test_with_nano() {
        for epoch in TEST_EPOCH {
            for ms in TEST_NS {
                let new = Epoch::new(epoch).with_nanos(ms);
                assert_eq!(new.precision, Precision::Nano(ms));
                assert_eq!(new.epoch, epoch);
            }
        }
    }

    #[test]
    #[should_panic(expected = "assertion failed: millis < 1000")]
    fn test_with_ms_panic() {
        Epoch::new(0).with_millis(1000);
    }

    #[test]
    #[should_panic(expected = "assertion failed: micros < 1000000")]
    fn test_with_micros_panic() {
        Epoch::new(0).with_micros(1000000);
    }

    #[test]
    #[should_panic(expected = "assertion failed: nanos < 1000000000")]
    fn test_with_nanos_panic() {
        Epoch::new(0).with_nanos(1000000000);
    }

    #[test]
    fn test_with_delimiter() {
        let delimiters = ['a', ':', '-'];
        for delimiter in delimiters {
            let epoch = Epoch::new(0).with_delimiter(delimiter);
            assert_eq!(epoch.delimiter, delimiter);
        }
    }

    #[test]
    fn test_default() {
        let default = Epoch::default();
        assert_eq!(default.epoch, 0);
        assert!(matches!(default.precision, Precision::None));
        assert_eq!(default.delimiter, '.');
    }

    #[test]
    fn test_display() {
        let epochs = [
            (0, "0"),
            (1, "1"),
            (-1, "-1"),
            (123, "123"),
            (-123, "-123"),
            (i64::MAX, "9223372036854775807"),
            (i64::MIN, "-9223372036854775808"),
        ];

        for (epoch, expected) in epochs {
            let epoch = Epoch::new(epoch);
            assert_eq!(epoch.to_string(), expected);
        }
    }

    #[test]
    fn test_display_with_millis() {
        let epochs = [
            (0, 0, "0.000"),
            (0, 999, "0.999"),
            (1, 123, "1.123"),
            (-1, 123, "-1.123"),
            (123, 999, "123.999"),
            (-123, 999, "-123.999"),
            (i64::MAX, 999, "9223372036854775807.999"),
            (i64::MIN, 999, "-9223372036854775808.999"),
        ];

        for (epoch, ms, expected) in epochs {
            let epoch = Epoch::new(epoch).with_millis(ms);
            assert_eq!(epoch.to_string(), expected);
        }
    }

    #[test]
    fn test_display_with_micros() {
        let epochs = [
            (0, 0, "0.000000"),
            (0, 999999, "0.999999"),
            (1, 123123, "1.123123"),
            (-1, 123123, "-1.123123"),
            (123, 999999, "123.999999"),
            (-123, 999999, "-123.999999"),
            (i64::MAX, 999999, "9223372036854775807.999999"),
            (i64::MIN, 999999, "-9223372036854775808.999999"),
        ];

        for (epoch, ms, expected) in epochs {
            let epoch = Epoch::new(epoch).with_micros(ms);
            assert_eq!(epoch.to_string(), expected);
        }
    }

    #[test]
    fn test_display_with_nanos() {
        let epochs = [
            (0, 0, "0.000000000"),
            (0, 999999999, "0.999999999"),
            (1, 123123123, "1.123123123"),
            (-1, 123123123, "-1.123123123"),
            (123, 999999999, "123.999999999"),
            (-123, 999999999, "-123.999999999"),
            (i64::MAX, 999999999, "9223372036854775807.999999999"),
            (i64::MIN, 999999999, "-9223372036854775808.999999999"),
        ];

        for (epoch, ms, expected) in epochs {
            let epoch = Epoch::new(epoch).with_nanos(ms);
            assert_eq!(epoch.to_string(), expected);
        }
    }

    #[test]
    fn test_display_with_delimiter() {
        let epochs = [
            (0, 0, '-', "0-000"),
            (0, 0, ':', "0:000"),
            (1, 0, ':', "1:000"),
            (-1, 0, ':', "-1:000"),
            (1, 999, ':', "1:999"),
            (-1, 999, ':', "-1:999"),
        ];

        for (epoch, ms, delimiter, expected) in epochs {
            let epoch = Epoch::new(epoch).with_millis(ms).with_delimiter(delimiter);
            assert_eq!(epoch.to_string(), expected);
        }
    }
}
