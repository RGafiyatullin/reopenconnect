use super::*;

#[derive(Debug, Clone)]
pub struct GroupAccess {
    pub uri: String,
    pub extra_attrs: HashMap<String, String>,
}

impl GroupAccess {
    pub fn new<S>(uri: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            uri: uri.into(),
            extra_attrs: Default::default(),
        }
    }
}

impl ToXmlSimple for GroupAccess {
    fn element_name(&self) -> String {
        str!("device-id")
    }
    fn element_attrs(&self) -> HashMap<String, String> {
        self.extra_attrs.to_owned()
    }
    fn element_children(&self) -> Vec<XmlNode> {
        vec![XmlNode::Text(self.uri.to_owned())]
    }
}
