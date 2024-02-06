use crate::common::{conf::Configuration, fs::permission::FsPermission};

/// DFSClient configuration.
pub struct DfsClientConf {
    umask: FsPermission,
}

impl DfsClientConf {
    pub fn new(conf: &Configuration) -> anyhow::Result<Self> {
        let umask = FsPermission::get_umask(Some(conf))?;
        Ok(Self { umask })
    }

    pub fn get_umask(&self) -> &FsPermission {
        &self.umask
    }
}
