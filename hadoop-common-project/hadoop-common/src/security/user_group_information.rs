use super::User;
use std::{collections::HashMap, env, sync::Mutex};

const HADOOP_PROXY_USER: &str = "HADOOP_PROXY_USER";

/// Information about the logged in user.
static LOGIN_USER_REF: Mutex<Option<UserGroupInformation>> = Mutex::new(None);

/// User and group information for Hadoop.
/// This class provides methods to determine the
/// user's username and groups. It supports both the Windows, Unix and Kerberos
/// login modules.
#[derive(Clone, Copy)]
pub struct UserGroupInformation {
    user: User,
}

impl UserGroupInformation {
    /// A method to initialize the fields that depend on a configuration.
    /// Must be called before useKerberos or groups is used.
    fn ensure_initialized() {
        // TODO
    }

    /// Return the current user, including any doAs in the current stack.
    pub fn get_current_user() -> anyhow::Result<Self> {
        Self::ensure_initialized();
        // TODO: use context user if any
        Self::get_login_user()
    }

    /// Get the currently logged in user.  If no explicit login has occurred,
    /// the user will automatically be logged in with either kerberos credentials
    /// if available, or as the local OS user, based on security settings.
    pub fn get_login_user() -> anyhow::Result<Self> {
        Self::ensure_initialized();
        // TODO: confirm whether to use Atomic or Mutex for LOGIN_USER_REF
        if let Some(login_user) = *LOGIN_USER_REF.lock().unwrap() {
            return Ok(login_user);
        }
        let new_login_user = Self::create_login_user(None)?;
        let login_user = *LOGIN_USER_REF.lock().unwrap().get_or_insert_with(|| {
            new_login_user.spawn_auto_renewal_thread_for_user_creds(false);
            new_login_user
        });
        Ok(login_user)
    }

    fn create_login_user(subject: Option<&str>) -> anyhow::Result<Self> {
        let real_user = Self::do_subject_login(subject, None)?;

        // If the HADOOP_PROXY_USER environment variable
        // is specified, create a proxy user as the logged in user.
        let proxy_user = env::var(HADOOP_PROXY_USER).ok().filter(|p| !p.is_empty());
        let login_user = proxy_user.map_or(real_user, |p| Self::create_proxy_user(&p, &real_user));

        // TODO: load tokens from files and base64 encoding

        Ok(login_user)
    }

    /// Spawn a thread to do periodic renewals of kerberos credentials. NEVER
    /// directly call this method. This method should only be used for ticket cache
    /// based kerberos credentials.
    fn spawn_auto_renewal_thread_for_user_creds(&self, _force: bool) {
        // TODO
    }

    /// Create a proxy user using username of the effective user and the ugi of the
    /// real user.
    pub fn create_proxy_user(_user: &str, real_user: &Self) -> Self {
        // TODO
        real_user.to_owned()
    }

    /// Get the user's login name.
    pub fn get_short_user_name(&self) -> String {
        self.user.get_short_name()
    }

    /// Get the user's full principal name.
    pub fn get_user_name(&self) -> String {
        self.user.get_name()
    }

    /// Login a subject with the given parameters.  If the subject is null,
    /// the login context used to create the subject will be attached.
    fn do_subject_login(
        _subject: Option<&str>,
        _params: Option<HashMap<String, String>>,
    ) -> anyhow::Result<Self> {
        // TODO
        Ok(Self { user: User {} })
    }
}
