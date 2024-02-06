use super::{ClientNamenodeProtocolPB, PBHelperClient};
use crate::{
    common::fs::permission::FsCreateModes,
    hdfs::{hdfs::protocol::ClientProtocol, *},
};

/// This class forwards NN's ClientProtocol calls as RPC calls to the NN server
/// while translating from the parameter types used in ClientProtocol to the
/// new PB types.
pub(crate) struct ClientNamenodeProtocolTranslatorPB {
    rpc_proxy: ClientNamenodeProtocolPB,
}

impl From<ClientNamenodeProtocolPB> for ClientNamenodeProtocolTranslatorPB {
    fn from(proxy: ClientNamenodeProtocolPB) -> Self {
        Self { rpc_proxy: proxy }
    }
}

impl ClientProtocol for ClientNamenodeProtocolTranslatorPB {
    fn mkdirs(
        &self,
        src: &str,
        create_modes: &FsCreateModes,
        create_parent: bool,
    ) -> anyhow::Result<bool> {
        let req = MkdirsRequestProto {
            src: src.to_owned(),
            masked: PBHelperClient::convert(create_modes.get_masked()),
            create_parent,
            unmasked: Some(PBHelperClient::convert(create_modes.get_unmasked())),
        };

        // TODO: unwrap service exception if any

        Ok(self.rpc_proxy.mkdirs(&req)?.result)
    }
}
