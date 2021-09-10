use super::*;

impl<IO, Ctx> Authenticator<IO, Ctx>
where
    IO: HttpIo,
{
    pub(super) async fn process_request(
        &mut self,
        http_request: HttpRequest<HttpBody>,
    ) -> Result<HttpResponse<HttpBody>, HttpError> {
        let response_fut = self
            .http_conn_api
            .send_request(http_request)
            .map_err(HttpError::ProcessRequest)
            .fuse();
        let conn_running = &mut self.http_conn_running;

        ::futures::pin_mut!(response_fut);
        ::futures::pin_mut!(conn_running);

        let http_response = ::futures::select_biased! {
            response = response_fut => response?,
            _conn_parts = conn_running => Err(HttpError::UnexpectedConnectionTermination)?,
        };

        Ok(http_response)
    }
}
