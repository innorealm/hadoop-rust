use super::{call::Call, Client, ConnectionId, IpcStreams};
use crate::common::{
    fs::common_configuration_keys::IPC_MAXIMUM_RESPONSE_LENGTH_DEFAULT,
    io::retry::RetryPolicy,
    ipc::{server::AuthProtocol, RpcConstants, RpcKind, RpcProtocol, RPC},
    rpc_request_header_proto::OperationProto,
    util::ProtoUtil,
};
use atomic::Atomic;
use prost::Message;
use std::{
    collections::HashMap,
    marker::PhantomData,
    net::{SocketAddr, TcpStream},
    rc::Rc,
    sync::Arc,
};

/// Thread that reads responses and notifies callers.  Each connection owns a
/// socket connected to a remote address.  Calls are multiplexed through this
/// socket: responses may be delivered out of order.
pub(super) struct Connection<'a, T: RpcProtocol> {
    client: &'a Client,
    _server: SocketAddr,
    remote_id: Rc<ConnectionId>,
    auth_method: String,
    auth_protocol: AuthProtocol,
    service_class: u8,
    pub ipc_streams: IpcStreams,
    _max_response_length: i32,
    _rpc_timeout: i32,
    _max_idle_time: i32,
    _connection_retry_policy: Option<Rc<dyn RetryPolicy>>,
    _max_retries_on_sasl: i32,
    _max_retries_on_socket_timeouts: i32,
    _tcp_no_delay: bool,
    _tcp_low_latency: bool,
    _do_ping: bool,
    _ping_interval: i32,
    _so_timeout: i32,
    _ping_request: Vec<u8>,
    // currently active calls
    calls: HashMap<i32, Rc<Call>>,
    phantom: PhantomData<T>,
}

impl<'a, T: RpcProtocol> Connection<'a, T> {
    pub fn new(
        client: &'a Client,
        remote_id: Rc<ConnectionId>,
        service_class: u8,
    ) -> anyhow::Result<Self> {
        let tcp_stream = TcpStream::connect(remote_id.get_address())?;
        let ipc_streams = IpcStreams::new(tcp_stream, IPC_MAXIMUM_RESPONSE_LENGTH_DEFAULT);
        // try SASL if security is enabled or if the ugi contains tokens.
        // this causes a SIMPLE client with tokens to attempt SASL
        // TODO: get value of try_sasl from UGI
        let try_sasl = false;
        let auth_protocol = if try_sasl {
            AuthProtocol::Sasl
        } else {
            AuthProtocol::None
        };
        Ok(Self {
            client,
            _server: remote_id.get_address().to_owned(),
            remote_id: Rc::clone(&remote_id),
            auth_method: "".to_string(),
            auth_protocol,
            service_class,
            ipc_streams,
            _max_response_length: IPC_MAXIMUM_RESPONSE_LENGTH_DEFAULT,
            _rpc_timeout: remote_id.get_rpc_timeout(),
            _max_idle_time: remote_id.get_max_idle_time(),
            _connection_retry_policy: remote_id.get_retry_policy(),
            _max_retries_on_sasl: remote_id.get_max_retries_on_sasl(),
            _max_retries_on_socket_timeouts: remote_id.get_max_retries_on_socket_timeouts(),
            _tcp_no_delay: remote_id.get_tcp_no_delay(),
            _tcp_low_latency: remote_id.get_tcp_low_latency(),
            _do_ping: remote_id.get_do_ping(),
            _ping_interval: remote_id.get_ping_interval(),
            _so_timeout: remote_id.get_rpc_timeout(),
            _ping_request: vec![],
            calls: HashMap::new(),
            phantom: PhantomData,
        })
    }

    /// Add a call to this connection's call queue and notify
    /// a listener; synchronized.
    pub(super) fn add_call(&mut self, call: Rc<Call>) {
        self.calls.insert(call.id, call);
    }

    pub fn setup_iostreams(
        &mut self,
        _fallback_to_simple_auth: Option<Arc<Atomic<bool>>>,
    ) -> anyhow::Result<()> {
        // TODO: implement missing details

        let remote_id = Rc::clone(&self.remote_id);
        let auth_method = &self.auth_method.clone();
        self.write_connection_header()?;
        if self.auth_protocol == AuthProtocol::Sasl {
            // TODO
            unimplemented!("SASL Auth is not implemented yet");
        }
        self.write_connection_context(remote_id, auth_method)?;
        Ok(())
    }

    /// Write the connection header - this is sent when connection is established
    /// +----------------------------------+
    /// |  "hrpc" 4 bytes                  |
    /// +----------------------------------+
    /// |  Version (1 byte)                |
    /// +----------------------------------+
    /// |  Service Class (1 byte)          |
    /// +----------------------------------+
    /// |  AuthProtocol (1 byte)           |
    /// +----------------------------------+
    fn write_connection_header(&mut self) -> anyhow::Result<()> {
        self.ipc_streams.send_request(RpcConstants::HEADER)?;
        self.ipc_streams.send_request(&[
            RpcConstants::CURRENT_VERSION,
            self.service_class,
            self.auth_protocol.call_id() as u8,
        ])?;
        Ok(())
    }

    /// Write the connection context header for each connection
    fn write_connection_context(
        &mut self,
        remote_id: Rc<ConnectionId>,
        auth_method: &str,
    ) -> anyhow::Result<()> {
        let message = ProtoUtil::make_ipc_connection_context(
            Some(RPC::get_protocol_name::<T>()),
            Some(remote_id.get_ticket()),
            auth_method,
        );
        let connection_context_header = ProtoUtil::make_rpc_request_header(
            &RpcKind::RpcProtocolBuffer,
            OperationProto::RpcFinalPacket,
            RpcConstants::CONNECTION_CONTEXT_CALL_ID,
            RpcConstants::INVALID_RETRY_COUNT,
            &self.client.client_id,
            None,
        );
        let mut buf = connection_context_header.encode_length_delimited_to_vec();
        buf.append(&mut message.encode_length_delimited_to_vec());
        let out = prepend_buf_size(buf);
        self.ipc_streams.send_request(&out)?;
        Ok(())
    }

    /// Initiates a rpc call by sending the rpc request to the remote server.
    /// Note: this is not called from the current thread, but by another
    /// thread, so that if the current thread is interrupted that the socket
    /// state isn't corrupted with a partially written message.
    pub fn send_rpc_request(&mut self, call: Rc<Call>) -> anyhow::Result<()> {
        let header = ProtoUtil::make_rpc_request_header(
            &call.rpc_kind,
            OperationProto::RpcFinalPacket,
            call.id,
            call.retry,
            &self.client.client_id,
            call.alignment_context.as_ref().map(Rc::clone),
        );
        let mut buf = header.encode_length_delimited_to_vec();
        buf.append(&mut call.rpc_request.as_ref().to_owned());
        let out = prepend_buf_size(buf);
        // TODO: rpc request queue
        self.ipc_streams.send_request(&out)?;
        Ok(())
    }
}

// TODO: consider implementing ResponseBuffer
fn prepend_buf_size(mut buf: Vec<u8>) -> Vec<u8> {
    let mut out = (buf.len() as i32).to_be_bytes().to_vec();
    out.append(&mut buf);
    out
}
