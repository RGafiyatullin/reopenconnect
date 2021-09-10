use crate::logging::LoggingConfig;
use crate::AnyError;

use super::common::*;
use super::Authenticate;
use super::AuthenticateAndConnect;
use super::Connect;

mod helpers;

mod context;
pub use context::Context;

mod run;

#[derive(Debug, Clone, ::structopt::StructOpt)]
pub struct ReOpenConnect {
    #[structopt(flatten)]
    pub logging: LoggingConfig,

    #[structopt(flatten)]
    pub add_http_headers: AddHttpHeadersSimple,

    #[structopt(flatten)]
    pub cert_store_args: CertStoreArgs,

    #[structopt(flatten)]
    pub connect_to: ConnectTo,

    #[structopt(subcommand)]
    pub sub: Sub,
}

#[derive(Debug, Clone, ::structopt::StructOpt)]
pub enum Sub {
    AuthenticateOnly(Authenticate),
    ConnectOnly(Connect),
    AuthenticateAndConnect(AuthenticateAndConnect),
}
