use super::*;

impl<IO, Ctx> Authenticator<IO, Ctx>
where
    IO: HttpIo,
{
    pub async fn into_io(self) -> Result<IO, AuthenticatorError> {
        let () = std::mem::drop(self.http_conn_api);

        let parts = self.http_conn_running.await?;

        assert!(parts.read_buf.is_empty());

        Ok(parts.io)
    }
}
