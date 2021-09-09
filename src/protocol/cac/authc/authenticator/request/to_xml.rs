use super::*;

pub trait ToXml: fmt::Debug + Send + Sync + 'static {
    fn to_xml(&self) -> XmlElement;
}

pub trait ToXmlSimple: Send + Sync + 'static {
    fn element_name(&self) -> String;
    fn element_attrs(&self) -> HashMap<String, String>;
    fn element_children(&self) -> Vec<XmlNode>;
}

impl<T> ToXml for T
where
    T: ToXmlSimple + fmt::Debug + Send + Sync + 'static,
{
    fn to_xml(&self) -> XmlElement {
        let mut builder = XmlElement::builder(self.element_name().as_str(), "");

        for (k, v) in self.element_attrs() {
            builder = builder.attr(k, v);
        }

        builder.append_all(self.element_children()).build()
    }
}

impl ToXml for XmlElement {
    fn to_xml(&self) -> XmlElement {
        self.to_owned()
    }
}
