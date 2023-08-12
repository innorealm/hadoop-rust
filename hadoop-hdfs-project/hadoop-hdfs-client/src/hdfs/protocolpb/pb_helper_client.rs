/// Utilities for converting protobuf classes to and from hdfs-client side
/// implementation classes and other helper utilities to help in dealing with
/// protobuf.
pub struct PBHelperClient;

impl PBHelperClient {
    pub fn convert<S: Into<T>, T>(value: S) -> T {
        value.into()
    }
}
