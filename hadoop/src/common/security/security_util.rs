use crate::common::io::Text;
use std::net::SocketAddr;

/// Security Utils.
pub struct SecurityUtil;

impl SecurityUtil {
    /// Construct the service key for a token
    pub fn build_token_service(addr: &SocketAddr) -> Text {
        // TODO: support flag `use_ip_for_token_service`

        format!("{}:{}", addr.ip(), addr.port()).into()
    }
}
