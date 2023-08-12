#[derive(Debug)]
pub struct Text {
    _bytes: Vec<u8>,
    _length: i32,
}

impl Text {
    /// Converts the provided String to bytes using the
    /// UTF-8 encoding. If `replace` is true, then
    /// malformed input is replaced with the
    /// substitution character, which is U+FFFD. Otherwise the
    /// method throws a MalformedInputException.
    fn encode(s: &str, _replace: bool) -> &[u8] {
        // TODO: handle malformed input

        s.as_bytes()
    }
}

impl From<String> for Text {
    fn from(s: String) -> Self {
        let bb = Self::encode(&s, true);
        Self {
            _bytes: bb.into(),
            _length: bb.len() as i32,
        }
    }
}
