use uuid::Uuid;

/// A class defining a set of static helper methods to provide conversion between
/// bytes and string for UUID-based client Id.
pub(crate) struct ClientId;

/// The byte array of a UUID should be 16
pub(crate) const BYTE_LENGTH: usize = 16;

impl ClientId {
    pub fn get_client_id() -> [u8; BYTE_LENGTH] {
        // TODO: confirm order of UUID MSB/LSB in result
        Uuid::new_v4().into_bytes()
    }
}
