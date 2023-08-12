#[derive(Clone, Copy)]
pub struct Configuration {}

impl Configuration {
    /// Get the value of the `name` property, `None` if
    /// no such property exists. If the key is deprecated, it returns the value of
    /// the first key which replaces the deprecated key and is not `None`.
    ///
    /// Values are processed for [`variable expansion`]
    /// before being returned.
    ///
    /// As a side effect get loads the properties from the sources if called for
    /// the first time as a lazy init.
    pub fn get<'a>(&self, _name: &str, default_value: Option<&'a str>) -> Option<&'a str> {
        // TODO
        None.or(default_value)
    }

    /// Get the value of the `name` property as a trimmed `&str`,
    /// `None` if no such property exists.
    /// If the key is deprecated, it returns the value of
    /// the first key which replaces the deprecated key and is not `None`
    ///
    /// Values are processed for [`variable expansion`]
    /// before being returned.
    pub fn get_trimmed<'a>(&self, name: &str) -> Option<&'a str> {
        self.get(name, None).map(|v| v.trim())
    }

    /// Get the value of the `name` property as a trimmed `&str`,
    /// `default_value` if no such property exists.
    /// See [`Configuration::get_trimmed`] for more details.
    pub fn get_trimmed_with_default<'a>(&self, name: &str, default_value: &'a str) -> &'a str {
        self.get_trimmed(name).unwrap_or(default_value)
    }

    /// Get the value of the `name` property as an `i32`.
    /// If no such property exists, the provided default value is returned,
    /// or if the specified value is not a valid `i32`,
    /// then an error is thrown.
    pub fn get_int(&self, name: &str, default_value: i32) -> anyhow::Result<i32> {
        if let Some(value_string) = self.get_trimmed(name) {
            Ok(i32::from_str_radix(value_string, 16)
                .or_else(|_| i32::from_str_radix(value_string, 10))?)
        } else {
            Ok(default_value)
        }
    }

    /// Get the value of the `name` property as an `i64`.
    /// If no such property exists, the provided default value is returned,
    /// or if the specified value is not a valid `i64`,
    /// then an error is thrown.
    pub fn get_long(&self, name: &str, default_value: i64) -> anyhow::Result<i64> {
        if let Some(value_string) = self.get_trimmed(name) {
            Ok(i64::from_str_radix(value_string, 16)
                .or_else(|_| i64::from_str_radix(value_string, 10))?)
        } else {
            Ok(default_value)
        }
    }

    /// Get the value of the `name` property as a `bool`.
    /// If no such property is specified, or if the specified value is not a valid
    /// `bool`, then `default_value` is returned.
    pub fn get_bool(&self, name: &str, default_value: bool) -> bool {
        match self.get_trimmed(name) {
            Some(v) if v.to_lowercase() == "true" => true,
            Some(v) if v.to_lowercase() == "false" => false,
            _ => default_value,
        }
    }
}
