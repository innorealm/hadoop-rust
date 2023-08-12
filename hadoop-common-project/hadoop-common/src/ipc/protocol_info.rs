/// The protocol name that is used when a client and server connect.
pub struct ProtocolInfo {
    // the name of the protocol (i.e. rpc service)
    pub protocol_name: &'static str,
    pub protocol_version: u64,
}
