/// Save the full and short name of the user as a principal. This allows us to
/// have a single type that we always look for when picking up user names.
#[derive(Clone, Copy)]
pub(crate) struct User {
    // TODO
}

impl User {
    /// Get the full name of the user.
    pub fn get_name(&self) -> String {
        // TODO
        whoami::username()
    }

    /// Get the user name up to the first '/' or '@'
    pub fn get_short_name(&self) -> String {
        // TODO
        whoami::username()
    }
}
