use super::*;

use std::collections::HashMap;

use super::common::*;
use crate::protocol::cac::authc::consts;

pub struct AuthConfigReply {
    pub extra_attrs: HashMap<String, String>,

    pub version: Version,
    pub device_id: DeviceID,
    pub group_access: GroupAccess,

    pub auth_reply_nodes: Vec<XmlElement>,

    pub extra_children: Vec<Box<dyn ToXml>>,
}

impl ToXmlSimple for AuthConfigReply {
    fn element_name(&self) -> String {
        str!(consts::CONFIG_AUTH)
    }
    fn element_attrs(&self) -> HashMap<String, String> {
        vec![
            (str!("client"), str!("vpn")),
            (
                str!("type"),
                str!(consts::config_auth_types::TYPE_AUTH_REPLY),
            ),
        ]
        .into_iter()
        .collect()
    }
    fn element_children(&self) -> Vec<XmlNode> {
        let auth = XmlElement::builder(consts::config_auth_request::AUTH, "")
            .append_all(self.auth_reply_nodes.to_owned())
            .build();
        vec![
            self.version.to_xml(),
            self.device_id.to_xml(),
            self.group_access.to_xml(),
            auth,
        ]
        .into_iter()
        .chain(self.extra_children.iter().map(|c| c.to_xml()))
        .map(XmlNode::Element)
        .collect()
    }
}

impl fmt::Debug for AuthConfigReply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dbg = f.debug_struct(std::any::type_name::<Self>());
        dbg.field("extra-attrs", &self.extra_attrs);

        for child in &self.extra_children {
            dbg.field("+", &child);
        }

        dbg.finish()
    }
}
