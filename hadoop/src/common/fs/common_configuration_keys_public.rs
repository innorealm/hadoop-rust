// FS keys
pub const FS_DEFAULT_NAME_KEY: &str = "fs.defaultFS";
/// Default value for [`FS_DEFAULT_NAME_KEY`]
pub const FS_DEFAULT_NAME_DEFAULT: &str = "file:///";

pub const FS_CLIENT_RESOLVE_REMOTE_SYMLINKS_KEY: &str = "fs.client.resolve.remote.symlinks";
/// Default value for [`FS_CLIENT_RESOLVE_REMOTE_SYMLINKS_KEY`]
pub const FS_CLIENT_RESOLVE_REMOTE_SYMLINKS_DEFAULT: bool = true;

pub const IPC_CLIENT_CONNECTION_MAXIDLETIME_KEY: &str = "ipc.client.connection.maxidletime";
/// Default value for [`IPC_CLIENT_CONNECTION_MAXIDLETIME_KEY`]
pub const IPC_CLIENT_CONNECTION_MAXIDLETIME_DEFAULT: i32 = 10000;

pub const IPC_CLIENT_CONNECT_TIMEOUT_KEY: &str = "ipc.client.connect.timeout";
/// Default value for [`IPC_CLIENT_CONNECT_TIMEOUT_KEY`]
pub const IPC_CLIENT_CONNECT_TIMEOUT_DEFAULT: i32 = 20000;

pub const IPC_CLIENT_CONNECT_MAX_RETRIES_ON_SOCKET_TIMEOUTS_KEY: &str =
    "ipc.client.connect.max.retries.on.timeouts";
/// Default value for [`IPC_CLIENT_CONNECT_MAX_RETRIES_ON_SOCKET_TIMEOUTS_KEY`]
pub const IPC_CLIENT_CONNECT_MAX_RETRIES_ON_SOCKET_TIMEOUTS_DEFAULT: i32 = 45;

pub const IPC_CLIENT_TCPNODELAY_KEY: &str = "ipc.client.tcpnodelay";
/// Default value for [`IPC_CLIENT_TCPNODELAY_KEY`]
pub const IPC_CLIENT_TCPNODELAY_DEFAULT: bool = true;

pub const IPC_CLIENT_LOW_LATENCY: &str = "ipc.client.low-latency";
/// Default value of [`IPC_CLIENT_LOW_LATENCY`]
pub const IPC_CLIENT_LOW_LATENCY_DEFAULT: bool = false;
