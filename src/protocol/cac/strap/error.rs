use ::base64::DecodeError as Base64DecodeError;
use ::openssl::error::ErrorStack as OpenSslErrorStack;

use super::OpenSslNid;

#[derive(Debug, ::thiserror::Error)]
pub enum StrapError {
    #[error("STRAP Error. OpenSSL")]
    OpenSsl(#[source] OpenSslErrorStack),

    #[error("STRAP Error. Base64")]
    Base64Decode(#[source] Base64DecodeError),

    #[error("STRAP Error. Bad NID: {:?}", _0)]
    BadMessageDigestNid(OpenSslNid),
}

impl From<OpenSslErrorStack> for StrapError {
    fn from(e: OpenSslErrorStack) -> Self {
        Self::OpenSsl(e)
    }
}
impl From<Base64DecodeError> for StrapError {
    fn from(e: Base64DecodeError) -> Self {
        Self::Base64Decode(e)
    }
}
