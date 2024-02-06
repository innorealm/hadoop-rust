use super::{
    protocol::ClientProtocol,
    protocolpb::{ClientNamenodeProtocolPB, ClientNamenodeProtocolTranslatorPB},
    DFSUtilClient,
};
use crate::common::{
    conf::Configuration,
    io::Text,
    ipc::{AlignmentContext, Client, RPC},
    security::{SecurityUtil, UserGroupInformation},
};
use atomic::Atomic;
use iref::Iri;
use std::{net::SocketAddr, rc::Rc, sync::Arc};

/// Wrapper for a client proxy as well as its associated service ID.
/// This is simply used as a tuple-like return type for created NN proxy.
pub(crate) struct ProxyAndInfo<PROXYTYPE> {
    pub proxy: PROXYTYPE,
    pub dt_service: Text,
    pub _address: SocketAddr,
}

/// Create proxy objects with `ClientProtocol` and
/// `HAServiceProtocol` to communicate with a remote NN. For the former,
/// generally use `NameNodeProxiesClient::create_proxy_with_client_protocol`,
/// which will create either an HA- or
/// non-HA-enabled client proxy as appropriate.
pub(crate) struct NameNodeProxiesClient;

impl NameNodeProxiesClient {
    /// Creates the namenode proxy with the ClientProtocol. This will handle
    /// creation of either HA- or non-HA-enabled proxy objects, depending upon
    /// if the provided URI is a configured logical URI.
    pub fn create_proxy_with_client_protocol(
        conf: &Configuration,
        name_node_uri: &Iri,
        fallback_to_simple_auth: Option<Arc<Atomic<bool>>>,
    ) -> anyhow::Result<ProxyAndInfo<impl ClientProtocol>> {
        // TODO: support HA proxy

        let nn_addr = DFSUtilClient::get_nnaddress(name_node_uri)?;
        let dt_service = SecurityUtil::build_token_service(&nn_addr);
        let proxy = Self::create_non_ha_proxy_with_client_protocol(
            &nn_addr,
            conf,
            &UserGroupInformation::get_current_user()?,
            true,
            fallback_to_simple_auth,
        )?;
        Ok(ProxyAndInfo {
            proxy,
            dt_service,
            _address: nn_addr,
        })
    }

    pub fn create_non_ha_proxy_with_client_protocol(
        address: &SocketAddr,
        conf: &Configuration,
        ugi: &UserGroupInformation,
        with_retries: bool,
        fallback_to_simple_auth: Option<Arc<Atomic<bool>>>,
    ) -> anyhow::Result<impl ClientProtocol> {
        Self::create_proxy_with_alignment_context(
            address,
            conf,
            ugi,
            with_retries,
            fallback_to_simple_auth,
            None,
        )
    }

    pub fn create_proxy_with_alignment_context(
        address: &SocketAddr,
        conf: &Configuration,
        ugi: &UserGroupInformation,
        with_retries: bool,
        fallback_to_simple_auth: Option<Arc<Atomic<bool>>>,
        alignment_context: Option<Rc<dyn AlignmentContext>>,
    ) -> anyhow::Result<impl ClientProtocol> {
        // TODO: set protocol engine for ClientNamenodeProtocolPB

        // TODO: get default policy

        let proxy: ClientNamenodeProtocolPB = RPC::get_protocol_proxy(
            address,
            ugi,
            conf,
            Client::get_timeout(conf),
            None,
            fallback_to_simple_auth,
            alignment_context,
        )?;

        if with_retries {
            // TODO: support retry proxy
        }

        Ok(ClientNamenodeProtocolTranslatorPB::from(proxy))
    }
}
