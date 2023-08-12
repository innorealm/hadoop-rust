use super::Client;
use crate::ipc::{AlignmentContext, RpcKind};
use std::rc::Rc;

/// Class that represents an RPC call
pub struct Call {
    // call id
    pub id: i32,
    // retry count
    pub retry: i32,
    // the serialized rpc request
    pub rpc_request: Rc<Vec<u8>>,
    // `None` if rpc has error
    _rpc_response: Option<String>,
    // exception, `None` if success
    _error: Option<String>,
    // Rpc EngineKind
    pub rpc_kind: RpcKind,
    // true when call is done
    _done: bool,
    _external_handler: Option<String>,
    pub alignment_context: Option<Rc<dyn AlignmentContext>>,
}

impl Call {
    pub(super) fn new(rpc_kind: &RpcKind, param: Rc<Vec<u8>>) -> anyhow::Result<Self> {
        Ok(Self {
            id: Client::take_call_id()?,
            retry: Client::get_retry_count()?,
            rpc_request: param,
            _rpc_response: None,
            _error: None,
            rpc_kind: rpc_kind.to_owned(),
            _done: false,
            _external_handler: Client::get_external_handler()?,
            alignment_context: None,
        })
    }

    /// Set an AlignmentContext for the call to update when call is done.
    pub fn set_alignment_context(&mut self, ac: Option<Rc<dyn AlignmentContext>>) {
        self.alignment_context = ac;
    }
}
