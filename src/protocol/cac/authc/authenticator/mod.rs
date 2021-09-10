use std::future::Future;
use std::pin::Pin;

use ::futures::prelude::*;
use ::hyper::client::conn::Parts as HttpConnParts;
use ::hyper::client::conn::SendRequest as HttpConnApi;
use ::hyper::Body as HttpBody;
use ::hyper::Method as HttpMethod;
use ::hyper::Request as HttpRequest;
use ::hyper::Response as HttpResponse;

type HttpConnRunning<IO> = ::futures::future::Fuse<
    Pin<Box<dyn Future<Output = Result<HttpConnParts<IO>, HttpError>> + Send>>,
>;

use crate::protocol::cac::ContextInvocationError;
use crate::protocol::cac::HttpError;
use crate::util::HttpIo;

use super::*;

mod error;
pub use error::AuthenticatorError;

mod fmt_debug;

mod create;
mod into_io;
mod run;

mod collect_body;
mod create_request;
mod http_handshake;
mod process_request;
mod run_init;
mod run_reply;

pub struct Authenticator<IO, Ctx>
where
    IO: HttpIo,
{
    context: Ctx,
    http_conn_api: HttpConnApi<HttpBody>,
    http_conn_running: HttpConnRunning<IO>,
}
