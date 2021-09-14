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

        log::trace!("on-connected handler: {:?}", self.on_connected());
        if let Some(on_connected) = self.on_connected() {
            log::info!("spawning the on-connected handler: {:?}", on_connected);
            let mut command = ::tokio::process::Command::new(on_connected);
            let _ = command.stdin(std::process::Stdio::piped());
            let mut child = command.spawn()?;
            let mut child_stdin = child
                .stdin
                .take()
                .ok_or_else(|| ::eyre::eyre!("Failed to capture child's stdin"))?;

            let cstp_props_serialized = ::serde_json::to_string_pretty(&cstp.cstp_props)?;
            let () = child_stdin
                .write_all(cstp_props_serialized.as_bytes())
                .await?;
            let () = child_stdin.flush().await?;
            let () = std::mem::drop(child_stdin);

            let child_exit_status = child.wait().await?;

            log::info!("on-connected handler exited: {:?}", child_exit_status);

            if !child_exit_status.success() {
                Err(::eyre::eyre!(
                    "on-connected handler exited with non-zero status: {:?}",
                    child_exit_status
                ))?
            }
        }

        let () = cstp_io.shutdown().await?;

        Ok(())
    }
}
