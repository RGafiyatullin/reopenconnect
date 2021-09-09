use std::collections::HashMap;
use std::fmt;

use ::minidom::Element as XmlElement;
use ::minidom::Node as XmlNode;

pub trait Request {}

pub mod init;
pub use init::InitRequest;

mod to_xml;
pub use to_xml::ToXml;
pub use to_xml::ToXmlSimple;
