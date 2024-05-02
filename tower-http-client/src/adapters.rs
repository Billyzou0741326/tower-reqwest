//! Adapters for various clients.

/// Adapter for [`reqwest`] client.
///
/// [`reqwest`]: https://crates.io/crates/reqwest
#[cfg(feature = "reqwest")]
pub mod reqwest {
    pub use tower_reqwest::{error, HttpClientLayer, HttpClientService};
}
