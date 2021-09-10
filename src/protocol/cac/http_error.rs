use ::hyper::http::Error as HyperHttpError;
use ::hyper::Error as HyperError;
// use ::hyper::header::InvalidHeaderName;
use ::hyper::header::InvalidHeaderValue;

#[derive(Debug, ::thiserror::Error)]
pub enum HttpError {
    #[error("HTTP Transport Error: HTTP-Connection unexpectedly terminated")]
    UnexpectedConnectionTermination,

    #[error("HTTP Transport Error: HTTP-Connection")]
    Connection(#[source] HyperError),

    #[error("HTTP Transport Error: HTTP-Handshake")]
    Handshake(#[source] HyperError),

    #[error("HTTP Transport Error: HTTP-Body")]
    Body(#[source] HyperError),

    #[error("HTTP Transport Error: Process Request")]
    ProcessRequest(#[source] HyperError),

    #[error("HTTP Transport Error: HTTP-Request")]
    Request(#[source] HyperHttpError),

    #[error("HTTP Transport Errir: Header Value")]
    HeaderValue(#[source] InvalidHeaderValue),
}
