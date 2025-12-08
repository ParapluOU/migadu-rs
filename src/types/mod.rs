use serde::{Deserialize, Deserializer};

mod alias;
mod forwarding;
mod identity;
mod mailbox;
mod rewrite;

pub use alias::*;
pub use forwarding::*;
pub use identity::*;
pub use mailbox::*;
pub use rewrite::*;

/// Deserializes a nullable boolean (null becomes false).
pub(crate) fn nullable_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<bool>::deserialize(deserializer).map(|opt| opt.unwrap_or(false))
}
