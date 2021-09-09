use super::*;

use ::hyper::client::conn as http_conn;

impl<IO, Ctx> Authenticator<IO, Ctx>
where
    IO: HttpIo,
{
    pub async fn create(io: IO, context: Ctx) -> Result<Self, AnyError> {
        let (http_conn_api, http_conn) = http_conn::handshake(io).await?;

        let http_conn_running = http_conn.without_shutdown().boxed().fuse();

        let authenticator = Self {
            context,
            http_conn_api,
            http_conn_running,
        };

        Ok(authenticator)
    }
}
