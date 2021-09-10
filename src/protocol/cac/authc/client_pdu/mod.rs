use std::collections::HashMap;
use std::fmt;

use crate::XmlElement;
use crate::XmlNode;

pub trait Request {}

pub mod common;

pub mod init;
pub use init::AuthConfigInit;

pub mod reply;
pub use reply::AuthConfigReply;

mod to_xml;
pub use to_xml::ToXml;
pub use to_xml::ToXmlSimple;
