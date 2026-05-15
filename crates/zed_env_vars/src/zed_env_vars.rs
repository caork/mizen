pub use env_var::{EnvVar, bool_env_var, env_var};
use std::sync::LazyLock;

/// Whether Mizen is running in stateless mode.
/// When true, Mizen will use in-memory databases instead of persistent storage.
pub static MIZEN_STATELESS: LazyLock<bool> = bool_env_var!("MIZEN_STATELESS");
