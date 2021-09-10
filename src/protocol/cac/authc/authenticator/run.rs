use crate::protocol::cac::authc::server_pdu::AuthConfigComplete;

use super::*;

impl<IO, Ctx> Authenticator<IO, Ctx>
where
    IO: HttpIo,
    Ctx: BorrowMutAuthenticatorContext,
{
    pub async fn run(&mut self) -> Result<AuthConfigComplete, AuthenticatorError> {
        let (mut headers, mut resp) = self.run_init().await?;

        while let Some(req) = resp.as_request() {
            let () = self
                .context
                .borrow_mut()
                .received_request(&headers, req)
                .map_err(ContextInvocationError)
                .await?;
            let (hs, r) = self.run_reply().await?;
            headers = hs;
            resp = r;
        }

        if let Some(complete) = resp.as_complete() {
            let () = self
                .context
                .borrow_mut()
                .received_complete(&headers, complete)
                .map_err(ContextInvocationError)
                .await?;

            Ok(complete.to_owned())
        } else {
            let unsupported = resp.as_unsupported().unwrap();
            log::error!("Unexpected: {}", String::from(unsupported));
            Err(AuthenticatorError::CACProtocol(::eyre::eyre!(
                "Unexpected response from server: {:#?}",
                resp
            )))
        }
    }
}
