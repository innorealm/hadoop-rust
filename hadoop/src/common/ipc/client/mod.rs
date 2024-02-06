mod call;
mod connection;
mod connection_id;

use super::{client_id::BYTE_LENGTH, AlignmentContext, ClientId, RpcKind, RpcProtocol};
use crate::common::{conf::Configuration, fs::common_configuration_keys};
use anyhow::Error;
use atomic::Atomic;
use call::Call;
use connection::Connection;
pub use connection_id::ConnectionId;
use std::{
    cell::RefCell,
    io::{Read, Write},
    net::TcpStream,
    rc::Rc,
    sync::{atomic::Ordering, Arc},
};

/// A counter for generating call IDs.
static CALL_ID_COUNTER: Atomic<i32> = Atomic::new(0);

thread_local! {
    static CALL_ID: RefCell<Option<i32>> = RefCell::new(None);
    static RETRY_COUNT: RefCell<Option<i32>> = RefCell::new(None);
    static EXTERNAL_CALL_HANDLER: RefCell<Option<String>> = RefCell::new(None);
}

/// A client for an IPC service.  IPC calls take a single [`Writable`] as a
/// parameter, and return a [`Writable`] as their value.  A service runs on
/// a port and is defined by a parameter class and a value class.
pub struct Client {
    _value_class: String,
    _conf: Configuration,
    _connection_timeout: i32,
    _fallback_allowed: bool,
    _bind_to_wild_card_address: bool,
    client_id: [u8; BYTE_LENGTH],
    _max_async_calls: i32,
}

impl Client {
    pub fn get_timeout(_conf: &Configuration) -> i32 {
        // TODO
        3000
    }

    fn _get_call_id() -> anyhow::Result<i32> {
        Ok(CALL_ID
            .try_with(|x| *x.borrow())?
            .unwrap_or_else(|| Self::next_call_id()))
    }

    fn take_call_id() -> anyhow::Result<i32> {
        Ok(CALL_ID
            .try_with(|x| x.take())?
            .unwrap_or_else(|| Self::next_call_id()))
    }

    fn get_retry_count() -> anyhow::Result<i32> {
        Ok(RETRY_COUNT.try_with(|x| *x.borrow())?.unwrap_or_default())
    }

    fn get_external_handler() -> anyhow::Result<Option<String>> {
        Ok(EXTERNAL_CALL_HANDLER.try_with(|x| (*x.borrow()).to_owned())?)
    }

    /// Get the ping interval from configuration;
    /// If not set in the configuration, return the default value.
    fn get_ping_interval(conf: &Configuration) -> anyhow::Result<i32> {
        conf.get_int(
            common_configuration_keys::IPC_PING_INTERVAL_KEY,
            common_configuration_keys::IPC_PING_INTERVAL_DEFAULT,
        )
    }

    fn create_call(&self, rpc_kind: &RpcKind, rpc_request: Rc<Vec<u8>>) -> anyhow::Result<Call> {
        Call::new(rpc_kind, rpc_request)
    }

    pub fn new(value_class: &str, conf: &Configuration) -> anyhow::Result<Self> {
        let connection_timeout = conf.get_int(
            common_configuration_keys::IPC_CLIENT_CONNECT_TIMEOUT_KEY,
            common_configuration_keys::IPC_CLIENT_CONNECT_TIMEOUT_DEFAULT,
        )?;
        let fallback_allowed = conf.get_bool(
            common_configuration_keys::IPC_CLIENT_FALLBACK_TO_SIMPLE_AUTH_ALLOWED_KEY,
            common_configuration_keys::IPC_CLIENT_FALLBACK_TO_SIMPLE_AUTH_ALLOWED_DEFAULT,
        );
        let bind_to_wild_card_address = conf.get_bool(
            common_configuration_keys::IPC_CLIENT_BIND_WILDCARD_ADDR_KEY,
            common_configuration_keys::IPC_CLIENT_BIND_WILDCARD_ADDR_DEFAULT,
        );
        let max_async_calls = conf.get_int(
            common_configuration_keys::IPC_CLIENT_ASYNC_CALLS_MAX_KEY,
            common_configuration_keys::IPC_CLIENT_ASYNC_CALLS_MAX_DEFAULT,
        )?;
        Ok(Self {
            _value_class: value_class.to_owned(),
            _conf: conf.to_owned(),
            _connection_timeout: connection_timeout,
            _fallback_allowed: fallback_allowed,
            _bind_to_wild_card_address: bind_to_wild_card_address,
            client_id: ClientId::get_client_id(),
            _max_async_calls: max_async_calls,
        })
    }

    /// Make a call, passing `rpc_request`, to the IPC server defined by
    /// `remote_id`, returning the rpc response.
    pub fn call<T: RpcProtocol>(
        &self,
        rpc_kind: &RpcKind,
        rpc_request: Rc<Vec<u8>>,
        remote_id: Rc<ConnectionId>,
        service_class: u8,
        fallback_to_simple_auth: Option<Arc<Atomic<bool>>>,
        alignment_context: Option<Rc<dyn AlignmentContext>>,
    ) -> anyhow::Result<Vec<u8>> {
        // TODO: return Writable

        let mut call = self.create_call(rpc_kind, rpc_request)?;
        call.set_alignment_context(alignment_context);
        let call = Rc::new(call);

        let mut connection = self.get_connection::<T>(
            remote_id,
            Rc::clone(&call),
            service_class,
            fallback_to_simple_auth,
        )?;

        connection.send_rpc_request(Rc::clone(&call))?;

        // TODO: support asynchronous mode

        let res: Vec<u8> = self.get_rpc_response(Rc::clone(&call), &mut connection, 0)?;
        Ok(res)
    }

    /// Get a connection from the pool, or create a new one and add it to the
    /// pool.  Connections to a given ConnectionId are reused.
    fn get_connection<'a, T: RpcProtocol>(
        &'a self,
        remote_id: Rc<ConnectionId>,
        call: Rc<Call>,
        service_class: u8,
        fallback_to_simple_auth: Option<Arc<Atomic<bool>>>,
    ) -> anyhow::Result<Connection<'a, T>> {
        // TODO: connection pool
        let mut connection = Connection::new(self, remote_id, service_class)?;

        connection.add_call(call);

        // If the server happens to be slow, the method below will take longer to
        // establish a connection.
        connection.setup_iostreams(fallback_to_simple_auth)?;
        Ok(connection)
    }

    fn get_rpc_response<T: RpcProtocol>(
        &self,
        _call: Rc<Call>,
        connection: &mut Connection<T>,
        _timeout: i64,
    ) -> anyhow::Result<Vec<u8>> {
        // TODO: get rpc response from call
        Ok(connection.ipc_streams.read_response()?)
    }

    /// Returns the next valid sequential call ID by incrementing an atomic counter
    /// and masking off the sign bit.  Valid call IDs are non-negative integers in
    /// the range [ 0, 2^31 - 1 ].  Negative numbers are reserved for special
    /// purposes.  The values can overflow back to 0 and be reused.  Note that prior
    /// versions of the client did not mask off the sign bit, so a server may still
    /// see a negative call ID if it receives connections from an old client.
    fn next_call_id() -> i32 {
        CALL_ID_COUNTER.fetch_add(1, Ordering::SeqCst) & 0x7FFFFFFF
    }
}

struct IpcStreams {
    inner: TcpStream,
    max_response_length: i32,
    first_response: bool,
}

impl IpcStreams {
    fn new(inner: TcpStream, max_response_length: i32) -> Self {
        IpcStreams {
            inner,
            max_response_length,
            first_response: true,
        }
    }

    fn read_i32(&mut self) -> anyhow::Result<i32> {
        let mut buf = [0; 4];
        self.inner.read_exact(&mut buf)?;
        Ok(i32::from_be_bytes(buf))
    }

    fn read_response(&mut self) -> anyhow::Result<Vec<u8>> {
        let length = self.read_i32()?;
        if self.first_response {
            self.first_response = false;
            if length == -1 {
                self.read_i32()?;
                return Err(Error::msg("Remote Exception"));
            }
        }
        if length <= 0 {
            return Err(Error::msg("RPC response has invalid length"));
        }
        if self.max_response_length > 0 && length > self.max_response_length {
            return Err(Error::msg("RPC response exceeds maximum data length"));
        }
        let mut buf = vec![0; length as usize];
        self.inner.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn send_request(&mut self, buf: &[u8]) -> anyhow::Result<usize> {
        Ok(self.inner.write(buf)?)
    }

    fn _flush(&mut self) -> anyhow::Result<()> {
        Ok(self.inner.flush()?)
    }
}
