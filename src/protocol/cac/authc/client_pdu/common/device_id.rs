use super::*;

use crate::protocol::cac::authc::consts;

#[derive(Debug, Clone)]
pub struct DeviceID {
    pub platform: String,
    pub extra_attrs: HashMap<String, String>,
}

impl DeviceID {
    pub fn new<S>(platform: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            platform: platform.into(),
            extra_attrs: Default::default(),
        }
    }
}

impl ToXmlSimple for DeviceID {
    fn element_name(&self) -> String {
        str!(consts::config_auth_std::DEVICE_ID)
    }
    fn element_attrs(&self) -> HashMap<String, String> {
        self.extra_attrs.to_owned()
    }
    fn element_children(&self) -> Vec<XmlNode> {
        vec![XmlNode::Text(self.platform.to_owned())]
    }
}
