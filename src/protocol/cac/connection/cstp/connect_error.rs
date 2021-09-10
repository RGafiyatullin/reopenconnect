use ::hyper::StatusCode as HttpStatus;

use crate::protocol::cac::ContextInvocationError;
use crate::protocol::cac::HttpError;
use crate::AnyError;

#[derive(Debug, ::thiserror::Error)]
pub enum CstpConnectError {
    #[error("CSTP Error: HTTP")]
    Http(#[source] HttpError),
    #[error("CSTP Error: HTTP-Header missing")]
    HttpHeaderMissing,
    #[error("CSTP Error: HTTP-Header contained non UTF-8 value")]
    HttpHeaderIsNotUtf8(#[source] std::str::Utf8Error),
    #[error("CSTP Error: Failed to parse header-value")]
    HeaderParseError(#[source] AnyError),
    #[error("CSTP Error: Context Invocation Error")]
    ContextInvocationError(#[source] ContextInvocationError),
    #[error("CSTP Error: Non 2xx HTTP-Status")]
    HttpNon2xx(HttpStatus),
}

impl From<HttpError> for CstpConnectError {
    fn from(e: HttpError) -> Self {
        Self::Http(e)
    }
}

impl From<ContextInvocationError> for CstpConnectError {
    fn from(e: ContextInvocationError) -> Self {
        Self::ContextInvocationError(e)
    }
}
