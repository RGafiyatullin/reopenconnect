use ::hyper::StatusCode as HttpStatus;

use crate::protocol::cac::ContextInvocationError;
use crate::protocol::cac::HttpError;
use crate::AnyError;

#[derive(Debug, ::thiserror::Error)]
pub enum AuthenticatorError {
    #[error("Authenticator Error: HTTP Transport")]
    Http(#[source] HttpError),

    #[error("Authenticator Error: Non-2xx response — {:?}", _0)]
    HttpNon2xx(HttpStatus),

    #[error("Authenticator Error: Body is not a UTF-8 string")]
    BodyIsNotUtf8(#[source] std::str::Utf8Error),

    #[error("Authenticator Error: Body is not a valid XML")]
    BodyIsNotXml(#[source] ::minidom::Error),

    #[error("Authenticator Error: Context")]
    Context(#[source] ContextInvocationError),

    #[error("Authenticator Error: CACProtocolError")]
    CACProtocol(#[source] AnyError),
}

impl From<ContextInvocationError> for AuthenticatorError {
    fn from(e: ContextInvocationError) -> Self {
        Self::Context(e)
    }
}

impl From<HttpError> for AuthenticatorError {
    fn from(e: HttpError) -> Self {
        Self::Http(e)
    }
}
