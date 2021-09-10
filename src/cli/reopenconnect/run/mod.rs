use crate::util::HttpIo;

use super::*;

use tokio::io::AsyncWriteExt;

mod authenticate;
mod configure_tls;
mod connect;
mod create_tundev;
mod open_io;

impl ReOpenConnect {
    pub async fn run(&self) -> Result<(), AnyError> {
        let () = self.logging.setup();

        let mut context = Context::new(&self)?;

        let tls_connector = configure_tls::run(&self.cert_store_args).await?;

        let (session_token, io, tun_dev_args) = match &self.sub {
            Sub::AuthenticateOnly(_sub) => {
                let io = open_io::run(&self.connect_to, &tls_connector).await?;
                let (_io, session_token) = authenticate::run(io, &mut context).await?;
                println!("SESSION-TOKEN: {}", session_token);
                return Ok(());
            }
            Sub::AuthenticateAndConnect(sub) => {
                let io = open_io::run(&self.connect_to, &tls_connector).await?;
                let (mut io, session_token) = authenticate::run(io, &mut context).await?;

                let io = if sub.restart_io_to_connect {
                    let () = io.shutdown().await?;
                    open_io::run(&self.connect_to, &tls_connector).await?
                } else {
                    io
                };

                (session_token, io, &sub.tun_dev)
            }
            Sub::ConnectOnly(sub) => {
                let io = open_io::run(&self.connect_to, &tls_connector).await?;
                (sub.session_token.to_owned(), io, &sub.tun_dev)
            }
        };

        let (cstp_io, mut cstp) = connect::run(io, &session_token, context).await?;

        let tun_dev = create_tundev::run(tun_dev_args, &cstp.cstp_props, &cstp.context).await?;

        let (mut cstp_io, _tun_dev) = cstp.run(cstp_io, tun_dev).await?;

        let () = cstp_io.shutdown().await?;

        Ok(())
    }
}
