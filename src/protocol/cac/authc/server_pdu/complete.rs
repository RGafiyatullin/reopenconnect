use std::collections::HashMap;
use std::convert::TryFrom;

use crate::protocol::cac::authc::consts;
use crate::AnyError;
use crate::XmlElement;

#[derive(Debug, Clone)]
pub struct AuthConfigComplete {
    pub extra_attrs: HashMap<String, String>,

    pub session_token: String,
    pub extra_children: Vec<XmlElement>,
}

impl TryFrom<XmlElement> for AuthConfigComplete {
    type Error = AnyError;

    fn try_from(value: XmlElement) -> Result<Self, Self::Error> {
        let mut extra_attrs = HashMap::new();
        let mut extra_children = Vec::new();
        let mut session_token_opt = None;
        for (k, v) in value.attrs() {
            let _ = extra_attrs.insert(k.to_owned(), v.to_owned());
        }
        for child in value.children() {
            if child.name() == consts::config_auth_complete::SESSION_TOKEN {
                session_token_opt = Some(child.text());
            } else {
                let () = extra_children.push(child.to_owned());
            }
        }

        let ret = Self {
            extra_attrs,
            session_token: session_token_opt
                .ok_or_else(|| ::eyre::eyre!("No session-token in `compelte`"))?,
            extra_children,
        };
        Ok(ret)
    }
}
