pub use super::common_configuration_keys_public::*;

/// Default umask for files created in HDFS
pub const FS_PERMISSIONS_UMASK_KEY: &str = "fs.permissions.umask-mode";
/// Default value for [`FS_PERMISSIONS_UMASK_KEY`]
pub const FS_PERMISSIONS_UMASK_DEFAULT: i32 = 0o22;

/// How often does RPC client send pings to RPC server
pub const IPC_PING_INTERVAL_KEY: &str = "ipc.ping.interval";
/// Default value for [`IPC_PING_INTERVAL_KEY`]
pub const IPC_PING_INTERVAL_DEFAULT: i32 = 60000; // 1min

/// Enables pings from RPC client to the server
pub const IPC_CLIENT_PING_KEY: &str = "ipc.client.ping";
/// Default value of [`IPC_CLIENT_PING_KEY`]
pub const IPC_CLIENT_PING_DEFAULT: bool = true;

/// Max response size a client will accept
pub const IPC_MAXIMUM_RESPONSE_LENGTH: &str = "ipc.maximum.response.length";
/// Default value for [`IPC_MAXIMUM_RESPONSE_LENGTH`]
pub const IPC_MAXIMUM_RESPONSE_LENGTH_DEFAULT: i32 = 128 * 1024 * 1024;

pub const IPC_CLIENT_ASYNC_CALLS_MAX_KEY: &str = "ipc.client.async.calls.max";
pub const IPC_CLIENT_ASYNC_CALLS_MAX_DEFAULT: i32 = 100;

pub const IPC_CLIENT_FALLBACK_TO_SIMPLE_AUTH_ALLOWED_KEY: &str =
    "ipc.client.fallback-to-simple-auth-allowed";
pub const IPC_CLIENT_FALLBACK_TO_SIMPLE_AUTH_ALLOWED_DEFAULT: bool = false;

pub const IPC_CLIENT_BIND_WILDCARD_ADDR_KEY: &str = "ipc.client.bind.wildcard.addr";
pub const IPC_CLIENT_BIND_WILDCARD_ADDR_DEFAULT: bool = false;

pub const IPC_CLIENT_CONNECT_MAX_RETRIES_ON_SASL_KEY: &str =
    "ipc.client.connect.max.retries.on.sasl";
pub const IPC_CLIENT_CONNECT_MAX_RETRIES_ON_SASL_DEFAULT: i32 = 5;
