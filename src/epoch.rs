#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Epoch {
    epoch: i64,
    ms: Option<u16>,
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
    ///
    /// # Panics
    /// Will panic if ms is >= 1000
    ///
    /// # Examples
    ///
    /// ```
    /// use epoch_archive::Epoch;
    ///
    /// let epoch = Epoch::new(1337).with_ms(123);
    /// assert_eq!(epoch.epoch(), 1337);
    /// assert_eq!(epoch.ms(), Some(123));
    /// ```
    pub fn with_ms(self, ms: u16) -> Self {
        assert!(ms < 1000);
        Self {
            ms: Some(ms),
            ..self
        }
    }

    /// Set the delimiter character that is used to separate the epoch value from the millisecond value.
    ///
    /// # Examples
    ///
    /// ```
    /// use epoch_archive::Epoch;
    ///
    /// let epoch = Epoch::new(1337).with_ms(123).with_delimiter(':');
    /// assert_eq!(epoch.to_string(), "1337:123");
    /// ```
    pub fn with_delimiter(self, delimiter: char) -> Self {
        Self { delimiter, ..self }
    }

    // --------------------------------
    // ---------- FORMATTING ----------
    // --------------------------------

    /// Returns the epoch value as a string including the millisecond value.
    ///
    /// # Examples
    ///
    /// ```
    /// use epoch_archive::Epoch;
    ///
    /// let epoch = Epoch::new(1337).with_ms(123);
    /// assert_eq!(epoch.format_precise(), "1337.123");
    /// ```
    pub fn format_precise(&self) -> String {
        self.to_string()
    }

    /// Returns the epoch value as a string, excluding the millisecond value
    ///
    /// # Examples
    ///
    /// ```
    /// use epoch_archive::Epoch;
    ///
    /// let epoch = Epoch::new(1337).with_ms(123);
    /// assert_eq!(epoch.format_seconds(), "1337");
    /// ```
    pub fn format_seconds(&self) -> String {
        self.epoch.to_string()
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
    pub fn ms(&self) -> Option<u16> {
        self.ms
    }

    /// Returns the delimiter character.
    pub fn delimiter(&self) -> char {
        self.delimiter
    }
}

impl std::fmt::Display for Epoch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:}{}{:03}",
            self.epoch,
            self.delimiter,
            self.ms.unwrap_or_default()
        )
    }
}

impl Default for Epoch {
    fn default() -> Self {
        Self {
            epoch: 0,
            ms: None,
            delimiter: '.',
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

    #[test]
    fn test_new() {
        for epoch in TEST_EPOCH {
            let new = Epoch::new(epoch);
            assert_eq!(new.epoch, epoch);
        }
    }

    #[test]
    fn test_with_ms() {
        for epoch in TEST_EPOCH {
            for ms in TEST_MS {
                let new = Epoch::new(epoch).with_ms(ms);
                assert_eq!(new.ms, Some(ms));
                assert_eq!(new.epoch, epoch);
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_with_ms_panic() {
        Epoch::new(0).with_ms(1000);
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
        assert!(default.ms.is_none());
        assert_eq!(default.delimiter, '.');
    }

    #[test]
    fn test_display() {
        let epochs = [
            (0, "0.000"),
            (1, "1.000"),
            (-1, "-1.000"),
            (123, "123.000"),
            (-123, "-123.000"),
            (i64::MAX, "9223372036854775807.000"),
            (i64::MIN, "-9223372036854775808.000"),
        ];

        for (epoch, expected) in epochs {
            let epoch = Epoch::new(epoch);
            assert_eq!(epoch.to_string(), expected);
        }
    }

    #[test]
    fn test_display_with_ms() {
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
            let epoch = Epoch::new(epoch).with_ms(ms);
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
            let epoch = Epoch::new(epoch).with_ms(ms).with_delimiter(delimiter);
            assert_eq!(epoch.to_string(), expected);
        }
    }
}
