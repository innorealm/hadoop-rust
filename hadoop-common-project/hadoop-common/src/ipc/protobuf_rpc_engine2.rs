use super::{AlignmentContext, Client, ConnectionId, RpcEngine, RpcProtocol, RPC};
use crate::{
    conf::Configuration,
    io::retry::RetryPolicy,
    ipc::RpcKind,
    proto::hadoop::common::{
        rpc_response_header_proto::RpcStatusProto, RequestHeaderProto, RpcResponseHeaderProto,
    },
    security::UserGroupInformation,
};
use anyhow::Error;
use atomic::Atomic;
use prost::Message;
use std::{marker::PhantomData, net::SocketAddr, rc::Rc, sync::Arc};

pub struct ProtobufRpcEngine2;

impl RpcEngine for ProtobufRpcEngine2 {
    fn get_proxy<T: RpcProtocol>(
        &self,
        addr: &SocketAddr,
        ticket: &UserGroupInformation,
        conf: &Configuration,
        rpc_timeout: i32,
        connection_retry_policy: Option<Rc<dyn RetryPolicy>>,
        fallback_to_simple_auth: Option<Arc<Atomic<bool>>>,
        alignment_context: Option<Rc<dyn AlignmentContext>>,
    ) -> anyhow::Result<T> {
        Ok(T::from(Invoker::from_socket_addr(
            addr,
            ticket,
            conf,
            rpc_timeout,
            connection_retry_policy,
            fallback_to_simple_auth,
            alignment_context,
        )?))
    }
}

pub struct Invoker<T: RpcProtocol> {
    remote_id: Rc<ConnectionId>,
    client: Client,
    client_protocol_version: u64,
    protocol_name: String,
    fallback_to_simple_auth: Option<Arc<Atomic<bool>>>,
    alignment_context: Option<Rc<dyn AlignmentContext>>,
    phantom: PhantomData<T>,
}

impl<T: RpcProtocol> Invoker<T> {
    pub fn from_socket_addr(
        addr: &SocketAddr,
        ticket: &UserGroupInformation,
        conf: &Configuration,
        rpc_timeout: i32,
        connection_retry_policy: Option<Rc<dyn RetryPolicy>>,
        fallback_to_simple_auth: Option<Arc<Atomic<bool>>>,
        alignment_context: Option<Rc<dyn AlignmentContext>>,
    ) -> anyhow::Result<Self> {
        let connection_id = Rc::new(ConnectionId::get_connection_id(
            addr,
            ticket,
            rpc_timeout,
            connection_retry_policy,
            conf,
        )?);
        Ok(Self::from_connection_id(
            connection_id,
            conf,
            fallback_to_simple_auth,
            alignment_context,
        )?)
    }

    /// This constructor takes a connection_id, instead of creating a new one.
    pub fn from_connection_id(
        conn_id: Rc<ConnectionId>,
        conf: &Configuration,
        fallback_to_simple_auth: Option<Arc<Atomic<bool>>>,
        alignment_context: Option<Rc<dyn AlignmentContext>>,
    ) -> anyhow::Result<Self> {
        // TODO: construct & cache client (or consider client singleton)

        // TODO: value_class

        Ok(Self {
            remote_id: conn_id,
            client: Client::new("value_class", conf)?,
            client_protocol_version: RPC::get_protocol_version::<T>(),
            protocol_name: RPC::get_protocol_name::<T>().to_owned(),
            fallback_to_simple_auth,
            alignment_context,
            phantom: PhantomData,
        })
    }

    fn construct_rpc_request_header(&self, method: &str) -> RequestHeaderProto {
        RequestHeaderProto {
            method_name: method.to_owned(),
            declaring_class_protocol_name: self.protocol_name.to_owned(),
            client_protocol_version: self.client_protocol_version,
        }
    }

    /// This is the client side invoker of RPC method.
    pub fn invoke<M: Default + Message>(
        &self,
        method: &str,
        the_request: &impl Message,
    ) -> anyhow::Result<M> {
        let val = self.client.call::<T>(
            &RpcKind::RpcProtocolBuffer,
            Rc::new(self.construct_rpc_request(method, the_request)),
            Rc::clone(&self.remote_id),
            RPC::RPC_SERVICE_CLASS_DEFAULT,
            self.fallback_to_simple_auth.as_ref().map(Arc::clone),
            self.alignment_context.as_ref().map(Rc::clone),
        )?;

        // TODO: support asynchronous mode

        self.get_return_message(method, &val)
    }

    fn construct_rpc_request(&self, method: &str, the_request: &impl Message) -> Vec<u8> {
        let rpc_request_header = self.construct_rpc_request_header(method);
        let mut output = rpc_request_header.encode_length_delimited_to_vec();
        let mut payload = the_request.encode_length_delimited_to_vec();
        output.append(&mut payload);
        output
    }

    fn get_return_message<M: Default + Message>(
        &self,
        _method: &str,
        buf: &Vec<u8>,
    ) -> anyhow::Result<M> {
        // TODO: use Writable

        let mut buffer = &buf[..];
        let header: RpcResponseHeaderProto = Message::decode_length_delimited(buffer)?;
        let status = header.status();
        if status == RpcStatusProto::Success {
            let header_len = header.encode_length_delimited_to_vec().len();
            buffer = &buf[header_len..];
            let res = M::decode_length_delimited(buffer)?;
            return Ok(res);
        }
        Err(Error::msg(format!("{:#?}", header)))
    }
}
