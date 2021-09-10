use std::collections::HashMap;
use std::convert::TryFrom;

use crate::protocol::cac::authc::consts;
use crate::AnyError;
use crate::XmlElement;

#[derive(Debug, Clone)]
pub struct AuthConfigRequest {
    pub extra_attrs: HashMap<String, String>,

    pub auth: XmlElement,
    pub extra_children: Vec<XmlElement>,
}

impl TryFrom<XmlElement> for AuthConfigRequest {
    type Error = AnyError;

    fn try_from(value: XmlElement) -> Result<Self, Self::Error> {
        let mut extra_attrs = HashMap::new();
        let mut extra_children = Vec::new();
        let mut auth_opt = None;
        for (k, v) in value.attrs() {
            let _ = extra_attrs.insert(k.to_owned(), v.to_owned());
        }
        for child in value.children() {
            if child.name() == consts::config_auth_request::AUTH {
                auth_opt = Some(child.to_owned());
            } else {
                let () = extra_children.push(child.to_owned());
            }
        }

        let ret = Self {
            extra_attrs,
            auth: auth_opt.ok_or_else(|| ::eyre::eyre!("No auth-section in `auth-request`"))?,
            extra_children,
        };
        Ok(ret)
    }
}
