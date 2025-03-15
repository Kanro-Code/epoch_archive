use crate::EpochError;

use std::str::FromStr;

const DELIMITER: char = '.';

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Epoch {
    epoch: i64,
    subsecond: SubSecond,
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
    #[must_use]
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
    #[must_use]
    pub fn with_epoch(self, epoch: i64) -> Self {
        Self { epoch, ..self }
    }

    /// Sets the millisecond value.
    /// If another subsecond is already set, this will override it.
    ///
    /// # Panics
    /// Will panic if ms is >= 1000
    ///
    /// # Examples
    ///
    /// ```
    /// use epoch_archive::{Epoch, SubSecond};
    ///
    /// let epoch = Epoch::new(0).with_millis(123);
    /// assert!(matches!(epoch.subsecond(), SubSecond::Milli(123)));
    /// ```
    #[must_use]
    pub fn with_millis(self, millis: u16) -> Self {
        assert!(millis < 1000, "assertion failed: millis < 1000");
        Self {
            subsecond: SubSecond::Milli(millis),
            ..self
        }
    }

    /// Sets the microsecond value.
    /// If another subsecond is already set, this will override it.
    ///
    /// # Panics
    /// Will panic if micros is >= 1000000
    ///
    /// # Examples
    ///
    /// ```
    /// use epoch_archive::{Epoch, SubSecond};
    ///
    /// let epoch = Epoch::new(0).with_micros(123);
    /// assert!(matches!(epoch.subsecond(), SubSecond::Micro(123)));
    /// ```
    #[must_use]
    pub fn with_micros(self, micros: u32) -> Self {
        assert!(micros < 1_000_000, "assertion failed: micros < 1000000");
        Self {
            subsecond: SubSecond::Micro(micros),
            ..self
        }
    }

    /// Sets the nanosecond value.
    /// If another subsecond is already set, this will override it.
    ///
    /// # Panics
    /// Will panic if ns is >= 1000000000
    ///
    /// # Examples
    ///
    /// ```
    /// use epoch_archive::{Epoch, SubSecond};
    ///
    /// let epoch = Epoch::new(0).with_nanos(123);
    /// assert!(matches!(epoch.subsecond(), SubSecond::Nano(123)));
    /// ```
    #[must_use]
    pub fn with_nanos(self, nanos: u64) -> Self {
        assert!(
            nanos < 1_000_000_000,
            "assertion failed: nanos < 1000000000"
        );
        Self {
            subsecond: SubSecond::Nano(nanos),
            ..self
        }
    }

    // -----------------------------
    // ---------- GETTERS ----------
    // -----------------------------

    /// Returns the epoch value.
    #[must_use]
    pub fn epoch(&self) -> i64 {
        self.epoch
    }

    /// Returns the optional millisecond value.
    ///
    /// If no value is present, this returns None.
    #[must_use]
    pub fn subsecond(&self) -> &SubSecond {
        &self.subsecond
    }

    /// Returns the epoch value as a string with the specified delimiter.
    #[must_use]
    pub fn format_with_delimiter(&self, delimiter: char) -> String {
        match self.subsecond {
            SubSecond::None => format!("{:}", self.epoch),
            SubSecond::Milli(ms) => format!("{:}{}{:03}", self.epoch, delimiter, ms),
            SubSecond::Micro(us) => format!("{:}{}{:06}", self.epoch, delimiter, us),
            SubSecond::Nano(ns) => format!("{:}{}{:09}", self.epoch, delimiter, ns),
        }
    }

    /// Returns the epoch value as a string.
    #[must_use]
    pub fn format(&self) -> String {
        Self::format_with_delimiter(self, DELIMITER)
    }
}

impl std::fmt::Display for Epoch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl Default for Epoch {
    fn default() -> Self {
        Self {
            epoch: 0,
            subsecond: SubSecond::None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SubSecond {
    None,
    Milli(u16),
    Micro(u32),
    Nano(u64),
}

impl FromStr for SubSecond {
    type Err = EpochError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.len() {
            3 => Ok(SubSecond::Milli(s.parse()?)),
            6 => Ok(SubSecond::Micro(s.parse()?)),
            9 => Ok(SubSecond::Nano(s.parse()?)),
            _ => Err(EpochError::InvalidSubSecond(s.to_string())),
        }
    }
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
    const TEST_US: [u32; 4] = [0, 1, 999_999, 123_123];
    const TEST_NS: [u64; 4] = [0, 1, 999_999_999, 123_123_123];

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
                assert_eq!(new.subsecond, SubSecond::Milli(ms));
                assert_eq!(new.epoch, epoch);
            }
        }
    }

    #[test]
    fn test_with_micro() {
        for epoch in TEST_EPOCH {
            for ms in TEST_US {
                let new = Epoch::new(epoch).with_micros(ms);
                assert_eq!(new.subsecond, SubSecond::Micro(ms));
                assert_eq!(new.epoch, epoch);
            }
        }
    }

    #[test]
    fn test_with_nano() {
        for epoch in TEST_EPOCH {
            for ms in TEST_NS {
                let new = Epoch::new(epoch).with_nanos(ms);
                assert_eq!(new.subsecond, SubSecond::Nano(ms));
                assert_eq!(new.epoch, epoch);
            }
        }
    }

    #[test]
    #[should_panic(expected = "assertion failed: millis < 1000")]
    #[allow(unused_must_use)]
    fn test_with_ms_panic() {
        Epoch::new(0).with_millis(1000);
    }

    #[test]
    #[should_panic(expected = "assertion failed: micros < 1000000")]
    #[allow(unused_must_use)]
    fn test_with_micros_panic() {
        Epoch::new(0).with_micros(1_000_000);
    }

    #[test]
    #[should_panic(expected = "assertion failed: nanos < 1000000000")]
    #[allow(unused_must_use)]
    fn test_with_nanos_panic() {
        Epoch::new(0).with_nanos(1_000_000_000);
    }

    #[test]
    fn test_default() {
        let default = Epoch::default();
        assert_eq!(default.epoch, 0);
        assert!(matches!(default.subsecond, SubSecond::None));
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
            (0, 999_999, "0.999999"),
            (1, 123_123, "1.123123"),
            (-1, 123_123, "-1.123123"),
            (123, 999_999, "123.999999"),
            (-123, 999_999, "-123.999999"),
            (i64::MAX, 999_999, "9223372036854775807.999999"),
            (i64::MIN, 999_999, "-9223372036854775808.999999"),
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
            (0, 999_999_999, "0.999999999"),
            (1, 123_123_123, "1.123123123"),
            (-1, 123_123_123, "-1.123123123"),
            (123, 999_999_999, "123.999999999"),
            (-123, 999_999_999, "-123.999999999"),
            (i64::MAX, 999_999_999, "9223372036854775807.999999999"),
            (i64::MIN, 999_999_999, "-9223372036854775808.999999999"),
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
            let epoch = Epoch::new(epoch).with_millis(ms);
            assert_eq!(epoch.format_with_delimiter(delimiter), expected);
        }
    }

    #[test]
    fn test_subsecond_from_str() {
        let epochs = [
            ("000", SubSecond::Milli(0)),
            ("999", SubSecond::Milli(999)),
            ("000000", SubSecond::Micro(0)),
            ("999999", SubSecond::Micro(999_999)),
            ("000000000", SubSecond::Nano(0)),
            ("999999999", SubSecond::Nano(999_999_999)),
        ];

        for (epoch, expected) in epochs {
            let epoch = SubSecond::from_str(epoch).unwrap();
            assert_eq!(epoch, expected);
        }
    }

    #[test]
    fn test_subsecond_from_str_error() {
        let epochs = [
            "1",
            "22",
            "4444",
            "55555",
            "7777777",
            "88888888",
            "1234567890",
            "-1",
            "-333",
            "-666666",
            "-999999999",
            "3.33",
            "-3.33",
            "aaa",
            "bbbbbb",
            "",
            " ",
            "00a",
            "000.000.000",
        ];

        for epoch in epochs {
            let epoch = SubSecond::from_str(epoch);
            assert!(epoch.is_err());
        }
    }
}
