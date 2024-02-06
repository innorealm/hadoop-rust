use super::{protocol::hdfs_constants, DFSClient, DFSUtilClient};
use crate::common::{
    conf::Configuration,
    fs::{permission::FsPermission, FileSystem, Path},
};
use anyhow::Error;
use iref::{Iri, IriBuf, IriRefBuf};
use std::str::FromStr;

pub(crate) struct DistributedFileSystem {
    conf: Configuration,
    working_dir: Path,
    uri: IriBuf,
    dfs: DFSClient,
    resolve_symlinks: bool,
}

impl DistributedFileSystem {
    /// Checks that the passed URI belongs to this filesystem and returns
    /// just the path component. Expects a URI with an absolute path.
    fn _get_path_name(&self, file: &Path) -> String {
        file.to_uri().to_string()
    }

    fn mkdirs_internal(
        &self,
        f: &Path,
        permission: Option<&FsPermission>,
        create_parent: bool,
    ) -> anyhow::Result<bool> {
        let abs_f = self.fix_relative_part(f)?;

        // TODO: FileSystemLinkResolver

        self.dfs
            .mkdirs(abs_f.to_uri().path().as_str(), permission, create_parent)
    }
}

impl FileSystem for DistributedFileSystem {
    type FileSystemImpl = Self;

    fn new(uri: &Iri, conf: &Configuration) -> anyhow::Result<Self> {
        let mut base_uri: IriBuf = IriBuf::from_scheme(uri.scheme().to_owned());
        base_uri.set_authority(Some(
            uri.authority()
                .ok_or(Error::msg(format!("Incomplete HDFS URI, no host: {}", uri)))?,
        ));
        let dfs = DFSClient::new(uri, conf)?;
        Ok(Self {
            conf: conf.to_owned(),
            working_dir: get_home_directory(conf, &dfs)?,
            uri: base_uri,
            dfs,
            resolve_symlinks: Self::get_resolve_symlinks(conf),
        })
    }

    fn resolve_symlinks(&self) -> bool {
        self.resolve_symlinks
    }

    fn get_scheme(&self) -> anyhow::Result<&str> {
        Ok(hdfs_constants::HDFS_URI_SCHEME)
    }

    fn get_uri(&self) -> &Iri {
        self.uri.as_iri()
    }

    fn get_working_directory(&self) -> &Path {
        &self.working_dir
    }

    fn get_home_directory(&self) -> anyhow::Result<Path> {
        get_home_directory(&self.conf, &self.dfs)
    }

    /// Create a directory and its parent directories.
    ///
    /// See [`FsPermission#apply_umask`] for details of how
    /// the permission is applied.
    fn mkdirs(&self, f: &Path, permission: Option<&FsPermission>) -> anyhow::Result<bool> {
        self.mkdirs_internal(f, permission, true)
    }
}

fn get_home_directory(conf: &Configuration, dfs: &DFSClient) -> anyhow::Result<Path> {
    // TODO: refactor path qualification
    Ok(Path::from(IriRefBuf::from_str(
        &DFSUtilClient::get_home_directory(Some(conf), &dfs.ugi),
    )?))
}
