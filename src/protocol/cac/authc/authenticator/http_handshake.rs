use super::*;

use ::hyper::client::conn as http_conn;

impl<IO, Ctx> Authenticator<IO, Ctx>
where
    IO: HttpIo,
{
    pub(super) async fn http_handshake(
        io: IO,
    ) -> Result<(HttpConnApi<HttpBody>, HttpConnRunning<IO>), HttpError> {
        let (http_conn_api, http_conn) = http_conn::Builder::new()
            // .http1_preserve_header_case(true)
            .http1_title_case_headers(true)
            .handshake(io)
            .await
            .map_err(HttpError::Handshake)?;

        let http_conn_running = http_conn
            .without_shutdown()
            .map_err(HttpError::Connection)
            .boxed()
            .fuse();

        let ret = (http_conn_api, http_conn_running);
        Ok(ret)
    }
}
