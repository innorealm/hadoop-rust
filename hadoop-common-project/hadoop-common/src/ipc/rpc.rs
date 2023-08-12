use super::{AlignmentContext, Invoker, ProtobufRpcEngine2, ProtocolInfo, RpcEngine};
use crate::{conf::Configuration, io::retry::RetryPolicy, security::UserGroupInformation};
use atomic::Atomic;
use std::{net::SocketAddr, rc::Rc, sync::Arc};

#[derive(Clone)]
#[repr(u8)]
pub enum RpcKind {
    // Used for built in calls by tests
    RpcBuiltin,
    // Use WritableRpcEngine
    RpcWritable,
    // Use ProtobufRpcEngine
    RpcProtocolBuffer,
}

pub trait RpcProtocol<T: RpcProtocol = Self> {
    fn get_protocol_info() -> &'static ProtocolInfo;

    fn from(invoker: Invoker<T>) -> Self;
}

/// A simple RPC mechanism.
pub struct RPC;

impl RPC {
    pub const RPC_SERVICE_CLASS_DEFAULT: u8 = 0;

    /// Get the protocol name.
    pub fn get_protocol_name<T: RpcProtocol>() -> &'static str {
        T::get_protocol_info().protocol_name
    }

    /// Get the protocol version from protocol class.
    pub fn get_protocol_version<T: RpcProtocol>() -> u64 {
        T::get_protocol_info().protocol_version
    }

    /// return the RpcEngine configured to handle a protocol
    fn get_protocol_engine<T: RpcProtocol>(_conf: &Configuration) -> impl RpcEngine {
        // TODO: get or create an RPC Engine in cache for Protocol `T`

        ProtobufRpcEngine2
    }

    /// Get a protocol proxy that contains a proxy connection to a remote server
    /// and a set of methods that are supported by the server.
    pub fn get_protocol_proxy<T: RpcProtocol>(
        addr: &SocketAddr,
        ticket: &UserGroupInformation,
        conf: &Configuration,
        rpc_timeout: i32,
        connection_retry_policy: Option<Rc<dyn RetryPolicy>>,
        fallback_to_simple_auth: Option<Arc<Atomic<bool>>>,
        alignment_context: Option<Rc<dyn AlignmentContext>>,
    ) -> anyhow::Result<T> {
        // TODO: init SaslRpcServer if needed

        Self::get_protocol_engine::<T>(conf).get_proxy(
            addr,
            ticket,
            conf,
            rpc_timeout,
            connection_retry_policy,
            fallback_to_simple_auth,
            alignment_context,
        )
    }
}
