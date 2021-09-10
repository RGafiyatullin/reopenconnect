use super::*;

use ::hyper::StatusCode as HttpStatus;

use crate::XmlElement;

impl<IO, Ctx> Authenticator<IO, Ctx>
where
    IO: HttpIo,
{
    pub(super) async fn collect_body_bytes(
        &mut self,
        http_body: HttpBody,
    ) -> Result<Vec<u8>, HttpError> {
        log::trace!("collecting http-body: {:#?}", http_body);
        let body_bytes_fut = ::hyper::body::to_bytes(http_body).map_err(HttpError::Body);
        let conn_running = &mut self.http_conn_running;

        ::futures::pin_mut!(body_bytes_fut);
        ::futures::pin_mut!(conn_running);

        ::futures::select! {
            body_bytes = body_bytes_fut => {
                let body_bytes = body_bytes?;
                Ok(body_bytes.to_vec())
            },
            http_conn_parts = conn_running => {
                let http_conn_parts = http_conn_parts?;
                let body_bytes = body_bytes_fut.await?;

                assert!(http_conn_parts.read_buf.is_empty());
                 let io: IO = http_conn_parts.io;

                let (http_conn_api, http_conn_running) = Self::http_handshake(io).await?;

                self.http_conn_running = http_conn_running;
                self.http_conn_api = http_conn_api;

                Ok(body_bytes.to_vec())
            }
        }
    }

    pub(super) async fn collect_body_xml(
        &mut self,
        status: HttpStatus,
        http_body: HttpBody,
    ) -> Result<XmlElement, AuthenticatorError> {
        let body_bytes = self.collect_body_bytes(http_body).await?;
        let body_xml = if status.is_success() {
            let body_bytes = body_bytes.to_vec();
            let body_str =
                std::str::from_utf8(&body_bytes).map_err(AuthenticatorError::BodyIsNotUtf8)?;
            log::trace!("{:?}", body_str);
            let body_xml: XmlElement = body_str
                .replace("<config-auth ", r#"<config-auth xmlns='' "#)
                .parse()
                .map_err(AuthenticatorError::BodyIsNotXml)?;
            log::trace!("{:#?}", body_xml);
            body_xml
        } else {
            if let Ok(s) = std::str::from_utf8(&body_bytes) {
                log::warn!("response-body: {:?}", s);
            } else {
                log::trace!("response-body(bytes): {:02x?}", body_bytes);
            }

            Err(AuthenticatorError::HttpNon2xx(status))?
        };
        Ok(body_xml)
    }
}
