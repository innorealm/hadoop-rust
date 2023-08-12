use super::{
    common_configuration_keys, common_configuration_keys_public, permission::FsPermission, Path,
};
use crate::conf::Configuration;
use anyhow::Error;
use iref::Iri;
use std::any::type_name;

pub const FS_DEFAULT_NAME_KEY: &str = common_configuration_keys::FS_DEFAULT_NAME_KEY;
pub const DEFAULT_FS: &str = common_configuration_keys::FS_DEFAULT_NAME_DEFAULT;
/// Prefix for trash directory.
pub const TRASH_PREFIX: &str = ".Trash";
pub const USER_HOME_PREFIX: &str = "/user";

/// An abstract base class for a fairly generic filesystem.  It
/// may be implemented as a distributed filesystem, or as a "local"
/// one that reflects the locally-connected disk.  The local version
/// exists for small Hadoop instances and for testing.
pub trait FileSystem {
    type FileSystemImpl: FileSystem;

    fn new(uri: &Iri, conf: &Configuration) -> anyhow::Result<Self::FileSystemImpl>;

    fn get_resolve_symlinks(conf: &Configuration) -> bool {
        conf.get_bool(
            common_configuration_keys_public::FS_CLIENT_RESOLVE_REMOTE_SYMLINKS_KEY,
            common_configuration_keys_public::FS_CLIENT_RESOLVE_REMOTE_SYMLINKS_DEFAULT,
        )
    }

    /// Get the default FileSystem URI from a configuration.
    fn get_default_uri(conf: &Configuration) -> anyhow::Result<Iri> {
        Ok(Iri::new(conf.get_trimmed_with_default(
            common_configuration_keys::FS_DEFAULT_NAME_KEY,
            common_configuration_keys::FS_DEFAULT_NAME_DEFAULT,
        ))?)
    }

    /// Should symbolic links be resolved by `FileSystemLinkResolver`.
    fn resolve_symlinks(&self) -> bool;

    /// Return the protocol scheme for this FileSystem.
    fn get_scheme(&self) -> anyhow::Result<&str> {
        Err(Error::msg(format!(
            "Not implemented by the {} FileSystem implementation",
            type_name::<Self>().split("::").last().unwrap_or_default()
        )))
    }

    /// Returns a URI which identifies this FileSystem.
    fn get_uri(&self) -> Iri;

    /// Return the current user's home directory in this FileSystem.
    /// The default implementation returns `"/user/$USER/"`.
    fn get_home_directory(&self) -> anyhow::Result<Path>;

    /// Get the current working directory for the given FileSystem
    fn get_working_directory(&self) -> &Path;

    /// Make the given file and all non-existent parents into
    /// directories. Has roughly the semantics of Unix `mkdir -p`.
    /// Existence of the directory hierarchy is not an error.
    fn mkdirs(&self, f: &Path, permission: Option<&FsPermission>) -> anyhow::Result<bool>;

    /// See [`FileContext#fix_relative_part`]
    fn fix_relative_part(&self, p: &Path) -> anyhow::Result<Path> {
        if p.is_uri_path_absolute() {
            Ok(p.clone())
        } else {
            Path::from_parent(self.get_working_directory(), p)
        }
    }
}
