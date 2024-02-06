/// This interface intends to align the state between client and server
/// via RPC communication.
///
/// This should be implemented separately on the client side and server side
/// and can be used to pass state information on RPC responses from server
/// to client.
pub trait AlignmentContext {
    // TODO
}
