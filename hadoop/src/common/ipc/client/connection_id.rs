use super::Client;
use crate::common::{
    conf::Configuration,
    fs::{common_configuration_keys, common_configuration_keys_public},
    io::retry::RetryPolicy,
    security::UserGroupInformation,
};
use anyhow::Error;
use std::{net::SocketAddr, rc::Rc};

/// This class holds the address and the user ticket. The client connections
/// to servers are uniquely identified by `<remoteAddress, protocol, ticket>`
pub struct ConnectionId {
    address: SocketAddr,
    ticket: UserGroupInformation,
    rpc_timeout: i32,
    // connections will be culled if it was idle for maxIdleTime msecs
    max_idle_time: i32,
    connection_retry_policy: Option<Rc<dyn RetryPolicy>>,
    max_retries_on_sasl: i32,
    // the max. no. of retries for socket connections on time out exceptions
    max_retries_on_socket_timeouts: i32,
    // if T then disable Nagle's Algorithm
    tcp_no_delay: bool,
    // if T then use low-delay QoS
    tcp_low_latency: bool,
    // do we need to send ping message
    do_ping: bool,
    // how often sends ping to the server in msecs
    ping_interval: i32,
    // used to get the expected kerberos principal name
    _conf: Configuration,
}

impl ConnectionId {
    fn new(
        address: &SocketAddr,
        ticket: &UserGroupInformation,
        rpc_timeout: i32,
        connection_retry_policy: Option<Rc<dyn RetryPolicy>>,
        conf: &Configuration,
    ) -> anyhow::Result<Self> {
        let max_idle_time = conf.get_int(
            common_configuration_keys_public::IPC_CLIENT_CONNECTION_MAXIDLETIME_KEY,
            common_configuration_keys_public::IPC_CLIENT_CONNECTION_MAXIDLETIME_DEFAULT,
        )?;
        let max_retries_on_sasl = conf.get_int(
            common_configuration_keys::IPC_CLIENT_CONNECT_MAX_RETRIES_ON_SASL_KEY,
            common_configuration_keys::IPC_CLIENT_CONNECT_MAX_RETRIES_ON_SASL_DEFAULT,
        )?;
        let max_retries_on_socket_timeouts = conf.get_int(
            common_configuration_keys_public::IPC_CLIENT_CONNECT_MAX_RETRIES_ON_SOCKET_TIMEOUTS_KEY,
            common_configuration_keys_public::IPC_CLIENT_CONNECT_MAX_RETRIES_ON_SOCKET_TIMEOUTS_DEFAULT,
        )?;
        let tcp_no_delay = conf.get_bool(
            common_configuration_keys_public::IPC_CLIENT_TCPNODELAY_KEY,
            common_configuration_keys_public::IPC_CLIENT_TCPNODELAY_DEFAULT,
        );
        let tcp_low_latency = conf.get_bool(
            common_configuration_keys_public::IPC_CLIENT_LOW_LATENCY,
            common_configuration_keys_public::IPC_CLIENT_LOW_LATENCY_DEFAULT,
        );
        let do_ping = conf.get_bool(
            common_configuration_keys::IPC_CLIENT_PING_KEY,
            common_configuration_keys::IPC_CLIENT_PING_DEFAULT,
        );
        Ok(Self {
            address: address.to_owned(),
            ticket: ticket.to_owned(),
            rpc_timeout,
            max_idle_time,
            connection_retry_policy,
            max_retries_on_sasl,
            max_retries_on_socket_timeouts,
            tcp_no_delay,
            tcp_low_latency,
            do_ping,
            ping_interval: if do_ping {
                Client::get_ping_interval(conf)?
            } else {
                0
            },
            _conf: conf.to_owned(),
        })
    }

    pub fn get_address(&self) -> &SocketAddr {
        &self.address
    }

    /// This is used to update the remote address when an address change is detected.
    fn _set_address(&mut self, address: &SocketAddr) -> anyhow::Result<()> {
        // TODO: compare hostname
        if self.address.port() != address.port() {
            return Err(Error::msg(format!(
                "Port must match: {} vs {}",
                self.address, address
            )));
        }
        self.address = address.to_owned();
        Ok(())
    }

    pub fn get_ticket(&self) -> &UserGroupInformation {
        &self.ticket
    }

    pub fn get_rpc_timeout(&self) -> i32 {
        self.rpc_timeout
    }

    pub fn get_max_idle_time(&self) -> i32 {
        self.max_idle_time
    }

    pub fn get_max_retries_on_sasl(&self) -> i32 {
        self.max_retries_on_sasl
    }

    pub fn get_max_retries_on_socket_timeouts(&self) -> i32 {
        self.max_retries_on_socket_timeouts
    }

    pub fn get_tcp_no_delay(&self) -> bool {
        self.tcp_no_delay
    }

    pub fn get_tcp_low_latency(&self) -> bool {
        self.tcp_low_latency
    }

    pub fn get_do_ping(&self) -> bool {
        self.do_ping
    }

    pub fn get_ping_interval(&self) -> i32 {
        self.ping_interval
    }

    pub fn get_retry_policy(&self) -> Option<Rc<dyn RetryPolicy>> {
        self.connection_retry_policy.as_ref().map(Rc::clone)
    }

    /// Returns a ConnectionId object.
    pub fn get_connection_id(
        addr: &SocketAddr,
        ticket: &UserGroupInformation,
        rpc_timeout: i32,
        connection_retry_policy: Option<Rc<dyn RetryPolicy>>,
        conf: &Configuration,
    ) -> anyhow::Result<Self> {
        // TODO: set connection_retry_policy if not yet

        Self::new(addr, ticket, rpc_timeout, connection_retry_policy, conf)
    }
}
