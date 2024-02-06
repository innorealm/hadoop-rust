use super::FsPermission;

/// A class that stores both masked and unmasked create modes
/// and is a drop-in replacement for masked permission.
pub struct FsCreateModes {
    masked: FsPermission,
    unmasked: FsPermission,
}

impl FsCreateModes {
    /// Create from unmasked mode and umask.
    pub fn apply_umask(mode: &FsPermission, umask: &FsPermission) -> Self {
        Self::create(&mode.apply_umask(umask), mode)
    }

    /// Create from masked and unmasked modes.
    pub fn create(masked: &FsPermission, unmasked: &FsPermission) -> Self {
        FsCreateModes {
            masked: masked.to_owned(),
            unmasked: unmasked.to_owned(),
        }
    }

    pub fn get_masked(&self) -> &FsPermission {
        &self.masked
    }

    pub fn get_unmasked(&self) -> &FsPermission {
        &self.unmasked
    }
}
