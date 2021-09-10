#[macro_use]
extern crate str_macro;

pub use ::eyre::Report as AnyError;
#[derive(Debug, ::thiserror::Error)]
#[error("AnyhowError")]
pub struct AnyhowError(#[source] pub ::anyhow::Error);

use ::minidom::Element as XmlElement;
use ::minidom::Node as XmlNode;

pub mod cli;

pub mod logging;
pub mod protocol;
pub mod util;

pub mod tun_dev;
