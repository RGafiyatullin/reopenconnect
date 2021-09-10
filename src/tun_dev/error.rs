use std::error::Error as StdError;

#[derive(Debug, ::thiserror::Error)]
#[error("Could not create tun-dev")]
pub struct TunDevInitError(#[source] pub Box<dyn StdError + Send + Sync>);
