use super::{fs_action::FSACTION_VALUES, FsAction, UmaskParser};
use crate::{
    common::{
        conf::Configuration,
        fs::common_configuration_keys::{FS_PERMISSIONS_UMASK_DEFAULT, FS_PERMISSIONS_UMASK_KEY},
    },
    hdfs::FsPermissionProto,
};
use once_cell::sync::Lazy;

const UMASK_LABEL: &str = FS_PERMISSIONS_UMASK_KEY;
const DEFAULT_UMASK: i32 = FS_PERMISSIONS_UMASK_DEFAULT;

/// Default permission for directory
pub static DIR_DEFAULT_PERM: Lazy<FsPermission> = Lazy::new(FsPermission::get_dir_default);
/// Default permission for file
pub static FILE_DEFAULT_PERM: Lazy<FsPermission> = Lazy::new(FsPermission::get_file_default);

#[derive(Clone, Copy)]
pub struct FsPermission {
    useraction: FsAction,
    groupaction: FsAction,
    otheraction: FsAction,
    sticky_bit: bool,
}

impl FsPermission {
    fn parse_short(n: i16) -> (FsAction, FsAction, FsAction, bool) {
        (
            FSACTION_VALUES[(n >> 6 & 7) as usize],
            FSACTION_VALUES[(n >> 3 & 7) as usize],
            FSACTION_VALUES[(n & 7) as usize],
            n >> 9 & 1 == 1,
        )
    }

    pub fn update_short(&mut self, n: i16) {
        let (u, g, o, sb) = Self::parse_short(n);
        self.useraction = u;
        self.groupaction = g;
        self.otheraction = o;
        self.sticky_bit = sb;
    }

    /// Encode the object to a short.
    pub fn to_short(&self) -> i16 {
        (if self.sticky_bit { 1 << 9 } else { 0 })
            | (self.useraction.ordinal() << 6) as i16
            | (self.groupaction.ordinal() << 3) as i16
            | self.otheraction.ordinal() as i16
    }

    /// Apply a umask to this permission and return a new one.
    ///
    /// The umask is used by create, mkdir, and other Hadoop filesystem operations.
    /// The mode argument for these operations is modified by removing the bits
    /// which are set in the umask.  Thus, the umask limits the permissions which
    /// newly created files and directories get.
    pub fn apply_umask(&self, umask: &FsPermission) -> Self {
        Self {
            useraction: self.useraction.and(&umask.useraction.not()),
            groupaction: self.groupaction.and(&umask.groupaction.not()),
            otheraction: self.otheraction.and(&umask.otheraction.not()),
            sticky_bit: false,
        }
    }

    /// Get the user file creation mask (umask)
    ///
    /// [`UMASK_LABEL`] config param has umask value that is either symbolic
    /// or octal.
    ///
    /// Symbolic umask is applied relative to file mode creation mask;
    /// the permission op characters '+' clears the corresponding bit in the mask,
    /// '-' sets bits in the mask.
    ///
    /// Octal umask, the specified bits are set in the file mode creation mask.
    pub fn get_umask(conf: Option<&Configuration>) -> anyhow::Result<Self> {
        let mut umask = DEFAULT_UMASK as i16;
        // To ensure backward compatibility first use the deprecated key.
        // If the deprecated key is not present then check for the new key
        if let Some(conf) = conf {
            if let Some(conf_umask) = conf.get(UMASK_LABEL, None) {
                umask = UmaskParser::new(conf_umask)?.get_umask();
            }
        }
        Ok(Self::from(umask))
    }

    /// Get the default permission for directory.
    pub fn get_dir_default() -> Self {
        Self::from(0o777)
    }

    /// Get the default permission for file.
    pub fn get_file_default() -> Self {
        Self::from(0o666)
    }
}

impl From<i16> for FsPermission {
    /// Construct by the given mode.
    fn from(mode: i16) -> Self {
        let (u, g, o, sb) = Self::parse_short(mode);
        Self {
            useraction: u,
            groupaction: g,
            otheraction: o,
            sticky_bit: sb,
        }
    }
}

impl Into<FsPermissionProto> for &FsPermission {
    fn into(self) -> FsPermissionProto {
        FsPermissionProto {
            perm: self.to_short() as u32,
        }
    }
}
