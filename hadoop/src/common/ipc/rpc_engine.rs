use super::{AlignmentContext, RpcProtocol};
use crate::common::{conf::Configuration, io::retry::RetryPolicy, security::UserGroupInformation};
use atomic::Atomic;
use std::{net::SocketAddr, rc::Rc, sync::Arc};

/// An RPC implementation.
pub trait RpcEngine {
    /// Construct a client-side proxy object.
    fn get_proxy<T: RpcProtocol>(
        &self,
        addr: &SocketAddr,
        ticket: &UserGroupInformation,
        conf: &Configuration,
        rpc_timeout: i32,
        connection_retry_policy: Option<Rc<dyn RetryPolicy>>,
        fallback_to_simple_auth: Option<Arc<Atomic<bool>>>,
        alignment_context: Option<Rc<dyn AlignmentContext>>,
    ) -> anyhow::Result<T>;
}
