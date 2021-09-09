use std::future::Future;
use std::pin::Pin;

use ::futures::prelude::*;
use ::hyper::client::conn::Parts as HttpConnParts;
use ::hyper::client::conn::SendRequest as HttpConnApi;
use ::hyper::Body as HttpBody;
use ::hyper::Error as HttpFailure;
use ::hyper::Method as HttpMethod;
use ::hyper::Request as HttpRequest;
use ::hyper::Response as HttpResponse;

use crate::util::HttpIo;
use crate::AnyError;

mod phase;
pub use phase::AuthenticationPhase;

pub mod request;
pub use request::Request;

mod context;
pub use context::*;

mod fmt_debug;

mod create;
mod run_init;

mod create_request;
mod process_request;

pub struct Authenticator<IO, Ctx>
where
    IO: HttpIo,
{
    context: Ctx,
    http_conn_api: HttpConnApi<HttpBody>,
    http_conn_running: ::futures::future::Fuse<
        Pin<Box<dyn Future<Output = Result<HttpConnParts<IO>, HttpFailure>> + Send>>,
    >,
}
