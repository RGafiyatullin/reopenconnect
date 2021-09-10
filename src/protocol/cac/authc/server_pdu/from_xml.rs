use std::convert::TryFrom;

use crate::protocol::cac::authc::consts;

use super::*;

use crate::AnyError;
use crate::XmlElement;

impl TryFrom<XmlElement> for AuthConfigServerPDU {
    type Error = AnyError;

    fn try_from(value: XmlElement) -> Result<Self, Self::Error> {
        if value.name() != consts::CONFIG_AUTH {
            Err(::eyre::eyre!(
                "Expected config-auth element at the top-level"
            ))?
        }
        if value.attr("client") != Some("vpn") {
            Err(::eyre::eyre!(
                "Expected config-auth's 'client' attribute's value to be 'vpn'"
            ))?
        }
        let ret = match value.attr("type") {
            Some(consts::config_auth_types::TYPE_AUTH_REQUEST) => {
                Self::Request(AuthConfigRequest::try_from(value)?)
            }
            Some(consts::config_auth_types::TYPE_COMPLETE) => {
                Self::Complete(AuthConfigComplete::try_from(value)?)
            }
            _ => Self::Unsupported(value),
        };
        Ok(ret)
    }
}
