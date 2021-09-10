use super::*;

impl<IO, Ctx> Authenticator<IO, Ctx>
where
    IO: HttpIo,
    Ctx: BorrowMutAuthenticatorContext,
{
    pub async fn create(io: IO, context: Ctx) -> Result<Self, AuthenticatorError> {
        let (http_conn_api, http_conn_running) = Self::http_handshake(io).await?;

        let authenticator = Self {
            context,
            http_conn_api,
            http_conn_running,
        };

        Ok(authenticator)
    }
}
