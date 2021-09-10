pub use ::openssl::nid::Nid as OpenSslNid;

use ::openssl::ec::EcKey;
use ::openssl::pkey::Private;

mod error;
pub use error::StrapError;

mod strap_impl;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct Strap {
    ec_key: EcKey<Private>,
    verify: Option<Vec<u8>>,
}
