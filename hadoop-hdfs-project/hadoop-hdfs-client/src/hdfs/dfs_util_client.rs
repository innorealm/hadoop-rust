use super::protocol::hdfs_constants;
use crate::hdfs::client::hdfs_client_config_keys;
use anyhow::Error;
use hadoop_common::{
    conf::Configuration,
    fs::{file_system, path},
    net::NetUtils,
    security::UserGroupInformation,
};
use iref::Iri;
use std::net::SocketAddr;

pub(crate) struct DFSUtilClient;

impl DFSUtilClient {
    pub fn get_nnaddress(filesystem_uri: &Iri) -> anyhow::Result<SocketAddr> {
        let authority = filesystem_uri.authority().ok_or(Error::msg(format!(
            "Invalid URI for NameNode address (check {}): {} has no authority.",
            file_system::FS_DEFAULT_NAME_KEY,
            filesystem_uri
        )))?;
        if !hdfs_constants::HDFS_URI_SCHEME.eq_ignore_ascii_case(&filesystem_uri.scheme().as_str())
        {
            return Err(Error::msg(format!(
                "Invalid URI for NameNode address (check {}): {} is not of scheme '{}'.",
                file_system::FS_DEFAULT_NAME_KEY,
                filesystem_uri,
                hdfs_constants::HDFS_URI_SCHEME
            )));
        }
        let port = authority
            .port()
            .and_then(|p| p.as_str().parse::<i32>().ok())
            .unwrap_or(hdfs_client_config_keys::DFS_NAMENODE_RPC_PORT_DEFAULT);
        NetUtils::create_socket_addr_for_host(authority.host().as_str(), port)
    }

    /// Returns current user home directory under a home directory prefix.
    /// The home directory prefix can be defined by
    /// [`hdfs_client_config_keys::DFS_USER_HOME_DIR_PREFIX_KEY`].
    /// User info is obtained from given [`UserGroupInformation`].
    pub fn get_home_directory(conf: Option<&Configuration>, ugi: &UserGroupInformation) -> String {
        let mut user_home_prefix = hdfs_client_config_keys::DFS_USER_HOME_DIR_PREFIX_DEFAULT;
        if let Some(conf) = conf {
            user_home_prefix = conf
                .get(
                    hdfs_client_config_keys::DFS_USER_HOME_DIR_PREFIX_KEY,
                    Some(hdfs_client_config_keys::DFS_USER_HOME_DIR_PREFIX_DEFAULT),
                )
                .unwrap_or(user_home_prefix);
        }
        format!(
            "{}{}{}",
            user_home_prefix,
            path::SEPARATOR,
            ugi.get_short_user_name()
        )
    }
}
