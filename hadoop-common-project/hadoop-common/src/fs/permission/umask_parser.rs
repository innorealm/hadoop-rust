pub struct UmaskParser {
    umask_mode: i16,
}

impl UmaskParser {
    pub fn new(_mode_str: &str) -> anyhow::Result<Self> {
        Ok(Self { umask_mode: 0o22 })
    }

    pub fn get_umask(&self) -> i16 {
        self.umask_mode
    }
}
