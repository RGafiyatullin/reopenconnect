use super::*;

use crate::protocol::cac::authc::client_pdu::ToXml;
use crate::util::HttpHeaders;

impl<IO, Ctx> Authenticator<IO, Ctx>
where
    IO: HttpIo,
{
    pub(super) fn create_request<P, B>(
        &mut self,
        method: HttpMethod,
        path: P,
        headers: HttpHeaders,
        body_opt: Option<B>,
    ) -> Result<HttpRequest<HttpBody>, HttpError>
    where
        P: AsRef<str>,
        B: ToXml,
    {
        let mut builder = HttpRequest::builder().method(method).uri(path.as_ref());

        if let Some(hs) = builder.headers_mut() {
            *hs = headers
        }

        let body = if let Some(body) = body_opt {
            let xml = body.to_xml();
            let xml = format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>{}"#,
                String::from(&xml).replace(r#" xmlns="""#, "")
            );

            log::trace!("BODY: {}", xml);

            HttpBody::from(xml)
        } else {
            HttpBody::empty()
        };

        let request = builder.body(body).map_err(HttpError::Request)?;

        Ok(request)
    }
}
