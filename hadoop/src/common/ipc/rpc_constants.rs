pub(crate) struct RpcConstants;

impl RpcConstants {
    pub const CONNECTION_CONTEXT_CALL_ID: i32 = -3;

    pub const INVALID_RETRY_COUNT: i32 = -1;

    /// The Rpc-connection header is as follows
    /// +----------------------------------+
    /// |  "hrpc" 4 bytes                  |
    /// +----------------------------------+
    /// |  Version (1 byte)                |
    /// +----------------------------------+
    /// |  Service Class (1 byte)          |
    /// +----------------------------------+
    /// |  AuthProtocol (1 byte)           |
    /// +----------------------------------+

    /// The first four bytes of Hadoop RPC connections
    pub const HEADER: &'static [u8] = "hrpc".as_bytes();

    // 1 : Introduce ping and server does not throw away RPCs
    // 3 : Introduce the protocol into the RPC connection header
    // 4 : Introduced SASL security layer
    // 5 : Introduced use of {@link ArrayPrimitiveWritable$Internal}
    //     in ObjectWritable to efficiently transmit arrays of primitives
    // 6 : Made RPC Request header explicit
    // 7 : Changed Ipc Connection Header to use Protocol buffers
    // 8 : SASL server always sends a final response
    // 9 : Changes to protocol for HADOOP-8990
    pub const CURRENT_VERSION: u8 = 9;
}
