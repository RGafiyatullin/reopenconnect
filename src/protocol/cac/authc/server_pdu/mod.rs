use crate::XmlElement;

pub mod request;
pub use request::AuthConfigRequest;

pub mod complete;
pub use complete::AuthConfigComplete;

mod from_xml;

#[derive(Debug, Clone)]
pub enum AuthConfigServerPDU {
    Request(AuthConfigRequest),
    Complete(AuthConfigComplete),
    Unsupported(XmlElement),
}

impl AuthConfigServerPDU {
    pub fn is_request(&self) -> bool {
        match self {
            Self::Request(_) => true,
            _ => false,
        }
    }
    pub fn is_complete(&self) -> bool {
        match self {
            Self::Complete(_) => true,
            _ => false,
        }
    }
    pub fn is_unsupported(&self) -> bool {
        match self {
            Self::Unsupported(_) => true,
            _ => false,
        }
    }

    pub fn as_request(&self) -> Option<&AuthConfigRequest> {
        match self {
            Self::Request(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn as_complete(&self) -> Option<&AuthConfigComplete> {
        match self {
            Self::Complete(inner) => Some(inner),
            _ => None,
        }
    }
    pub fn as_unsupported(&self) -> Option<&XmlElement> {
        match self {
            Self::Unsupported(inner) => Some(inner),
            _ => None,
        }
    }
}
