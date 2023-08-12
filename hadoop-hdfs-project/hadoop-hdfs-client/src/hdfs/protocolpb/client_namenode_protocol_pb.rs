use crate::hdfs::protocol::hdfs_constants::CLIENT_NAMENODE_PROTOCOL_NAME;
use crate::proto::hadoop::hdfs::*;
use hadoop_common::ipc::{Invoker, ProtocolInfo, RpcProtocol};

/// Protocol that clients use to communicate with the NameNode.
pub(crate) struct ClientNamenodeProtocolPB {
    invoker: Invoker<ClientNamenodeProtocolPB>,
}

impl RpcProtocol for ClientNamenodeProtocolPB {
    fn get_protocol_info() -> &'static ProtocolInfo {
        static PROTOCOL_INFO: ProtocolInfo = ProtocolInfo {
            protocol_name: CLIENT_NAMENODE_PROTOCOL_NAME,
            protocol_version: 1,
        };
        &PROTOCOL_INFO
    }

    fn from(invoker: Invoker<Self>) -> Self {
        Self { invoker }
    }
}

macro_rules! client_namenode_protocol_method {
    ($method:ident, $req_type:ident, $res_type:ident) => {
        pub fn $method(&self, req: &$req_type) -> anyhow::Result<$res_type> {
            self.invoker.invoke(stringify!($method), req)
        }
    };
}

impl ClientNamenodeProtocolPB {
    client_namenode_protocol_method!(mkdirs, MkdirsRequestProto, MkdirsResponseProto);
}
