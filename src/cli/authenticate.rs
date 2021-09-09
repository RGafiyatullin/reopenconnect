use ::tokio::net::TcpStream;
use ::webpki::DNSNameRef;

use crate::logging::LoggingConfig;
use crate::protocol::cac::authc::Authenticator;
use crate::protocol::cac::authc::AuthenticatorContext;
use crate::protocol::cac::authc::AuthenticatorContextInit;
use crate::protocol::tls::CARootsConfig;
use crate::AnyError;

use super::ConnectTo;

#[derive(Debug, ::structopt::StructOpt)]
pub struct Authenticate {
    #[structopt(flatten)]
    connect_to: ConnectTo,

    #[structopt(flatten)]
    ca_roots_config: CARootsConfig,

    #[structopt(flatten)]
    logging: LoggingConfig,
}

impl Authenticate {
    pub async fn run(&self) -> Result<(), AnyError> {
        let tls_connector = self.ca_roots_config.create_tls_connector()?;
        let tls_domain = DNSNameRef::try_from_ascii_str(&self.connect_to.host)?;

        let tcp_stream = TcpStream::connect(&self.connect_to.address).await?;
        let tls_stream = tls_connector.connect(tls_domain, tcp_stream).await?;

        let mut authc_ctx = AuthcCtx {
            http_host: self.connect_to.host.to_owned(),
        };
        let mut authenticator = {
            let authc_ctx: &mut dyn AuthenticatorContext = &mut authc_ctx;
            Authenticator::create(tls_stream, authc_ctx).await?
        };

        let _init_result = authenticator.run_init().await?;

        unimplemented!("{:#?}", self)
    }
}

#[derive(Debug)]
struct AuthcCtx {
    http_host: String,
}

use crate::protocol::cac::authc::request;

#[async_trait::async_trait]
impl AuthenticatorContext for AuthcCtx {
    async fn http_host(&mut self) -> String {
        self.http_host.to_owned()
    }
}
#[async_trait::async_trait]
impl AuthenticatorContextInit for AuthcCtx {
    async fn group_access(&mut self) -> request::init::GroupAccess {
        let uri = format!("https://{}/", self.http_host);
        request::init::GroupAccess::new(uri)
    }
}
