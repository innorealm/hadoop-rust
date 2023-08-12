use crate::proto::hadoop::hdfs::*;
use hadoop_common::fs::permission::FsPermission;

/// Utilities for converting protobuf classes to and from hdfs-client side
/// implementation classes and other helper utilities to help in dealing with
/// protobuf.
pub struct PBHelperClient;

impl PBHelperClient {
    pub fn convert<S, T: From<S>>(value: S) -> T {
        T::from(value)
    }
}

impl From<&FsPermission> for FsPermissionProto {
    fn from(p: &FsPermission) -> Self {
        FsPermissionProto {
            perm: p.to_short() as u32,
        }
    }
}
