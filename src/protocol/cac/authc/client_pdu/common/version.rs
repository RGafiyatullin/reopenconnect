use super::*;

use crate::protocol::cac::authc::consts;

#[derive(Debug, Clone)]
pub struct Version {
    pub who: String,
    pub ver: String,
    pub extra_attrs: HashMap<String, String>,
}

impl Version {
    pub fn new<S>(ver: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            who: str!("vpn"),
            ver: ver.into(),
            extra_attrs: Default::default(),
        }
    }
}

impl ToXmlSimple for Version {
    fn element_name(&self) -> String {
        str!(consts::config_auth_std::VERSION)
    }
    fn element_attrs(&self) -> HashMap<String, String> {
        let mut extra_attrs = self.extra_attrs.to_owned();
        extra_attrs.insert("who".to_owned(), self.who.to_owned());
        extra_attrs
    }
    fn element_children(&self) -> Vec<XmlNode> {
        vec![XmlNode::Text(self.ver.to_owned())]
    }
}
