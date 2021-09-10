use ::hyper::Uri as HttpUri;

use crate::AnyError;

#[derive(Debug, Clone, ::structopt::StructOpt)]
pub struct ConnectTo {
    #[structopt(long = "connect-to")]
    pub uri: HttpUri,

    #[structopt(long = "http-host")]
    pub http_host: Option<String>,

    #[structopt(long = "debug-no-tls")]
    pub debug_no_tls: bool,
}

impl ConnectTo {
    pub fn http_host(&self) -> Result<&str, AnyError> {
        if let Some(overriden) = self.http_host.as_ref() {
            Ok(overriden)
        } else {
            let from_uri = self
                .uri
                .host()
                .ok_or(::eyre::eyre!("The provided URI has no HOST"))?;
            Ok(from_uri)
        }
    }

    pub fn connect_to_host(&self) -> Result<&str, AnyError> {
        let host = self
            .uri
            .host()
            .ok_or(::eyre::eyre!("The provided URI has no HOST"))?;
        Ok(host)
    }

    pub fn connect_to_port(&self) -> u16 {
        self.uri.port_u16().unwrap_or(443)
    }
}
