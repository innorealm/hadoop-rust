use crate::common::fs::permission::FsCreateModes;

/// Until version 69, this class ClientProtocol served as both
/// the client interface to the NN AND the RPC protocol used to
/// communicate with the NN.
///
/// This class is used by both the DFSClient and the
/// NN server side to insulate from the protocol serialization.
///
/// If you are adding/changing this interface then you need to
/// change both this class and ALSO related protocol buffer
/// wire protocol definition in ClientNamenodeProtocol.proto.
///
/// For more details on protocol buffer wire protocol, please see
/// .../org/apache/hadoop/hdfs/protocolPB/overview.html
///
/// The log of historical changes can be retrieved from the svn).
/// 69: Eliminate overloaded method names.
///
/// 69L is the last version id when this class was used for protocols
///  serialization. DO not update this version any further.
pub(crate) const _VERSION_ID: u64 = 69;

/// ClientProtocol is used by user code via the DistributedFileSystem class to
/// communicate with the NameNode.  User code can manipulate the directory
/// namespace, as well as open/close file streams, etc.
pub(crate) trait ClientProtocol {
    /// Create a directory (or hierarchy of directories) with the given
    /// name and permission.
    fn mkdirs(
        &self,
        src: &str,
        create_modes: &FsCreateModes,
        create_parent: bool,
    ) -> anyhow::Result<bool>;
}
