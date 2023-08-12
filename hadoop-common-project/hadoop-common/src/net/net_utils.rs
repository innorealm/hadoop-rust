use anyhow::Error;
use std::net::{SocketAddr, ToSocketAddrs};

pub struct NetUtils;

impl NetUtils {
    pub fn create_socket_addr_for_host(host: &str, port: i32) -> anyhow::Result<SocketAddr> {
        // TODO: resolve host

        Ok(format!("{}:{}", host, port)
            .to_socket_addrs()?
            .filter(|s| s.is_ipv4())
            .last()
            .ok_or(Error::msg(format!(
                "Does not contain a valid host:port authority: {}:{}",
                host, port
            )))?)
    }
}
