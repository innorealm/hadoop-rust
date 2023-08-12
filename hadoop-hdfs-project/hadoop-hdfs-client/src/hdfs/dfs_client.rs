use super::{client::r#impl::DfsClientConf, protocol::ClientProtocol, NameNodeProxiesClient};
use atomic::Atomic;
use hadoop_common::{
    conf::Configuration,
    fs::permission::{fs_permission::DIR_DEFAULT_PERM, FsCreateModes, FsPermission},
    io::Text,
    security::UserGroupInformation,
};
use iref::Iri;
use std::sync::Arc;

/// DFSClient can connect to a Hadoop Filesystem and
/// perform basic file tasks.  It uses the ClientProtocol
/// to communicate with a NameNode daemon, and connects
/// directly to DataNodes to read/write block data.
///
/// Hadoop DFS users should obtain an instance of
/// DistributedFileSystem, which uses DFSClient to handle
/// filesystem tasks.
pub(crate) struct DFSClient {
    _conf: Configuration,
    dfs_client_conf: DfsClientConf,
    namenode: Box<dyn ClientProtocol>,
    // The service used for delegation tokens
    _dt_service: Text,
    pub ugi: UserGroupInformation,
}

impl DFSClient {
    /// Create a new DFSClient connected to the given nameNodeUri or rpcNamenode.
    pub fn new(name_node_uri: &Iri, conf: &Configuration) -> anyhow::Result<Self> {
        let nn_fallback_to_simple_auth = Arc::new(Atomic::new(false));
        let proxy_info = NameNodeProxiesClient::create_proxy_with_client_protocol(
            conf,
            name_node_uri,
            Some(nn_fallback_to_simple_auth),
        )?;
        Ok(Self {
            _conf: conf.to_owned(),
            dfs_client_conf: DfsClientConf::new(&conf)?,
            namenode: Box::new(proxy_info.proxy),
            _dt_service: proxy_info.dt_service,
            ugi: UserGroupInformation::get_current_user()?,
        })
    }

    fn apply_umask_dir(&self, permission: Option<&FsPermission>) -> FsCreateModes {
        let permission = permission.unwrap_or_else(|| &DIR_DEFAULT_PERM);
        FsCreateModes::apply_umask(permission, self.dfs_client_conf.get_umask())
    }

    /// Create a directory (or hierarchy of directories) with the given
    /// name and permission.
    pub fn mkdirs(
        &self,
        src: &str,
        permission: Option<&FsPermission>,
        create_parent: bool,
    ) -> anyhow::Result<bool> {
        let masked = self.apply_umask_dir(permission);
        self.primitive_mkdir(src, &masked, create_parent)
    }

    /// Same [`mkdirs`] except
    /// that the permissions has already been masked against umask.
    pub fn primitive_mkdir(
        &self,
        src: &str,
        create_modes: &FsCreateModes,
        create_parent: bool,
    ) -> anyhow::Result<bool> {
        self.namenode.mkdirs(src, create_modes, create_parent)
    }
}
