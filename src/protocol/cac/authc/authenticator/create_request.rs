use super::*;

use super::request::ToXml;

impl<IO, Ctx> Authenticator<IO, Ctx>
where
    IO: HttpIo,
{
    pub(super) fn create_request<P, Hs, B, Hk, Hv>(
        &mut self,
        method: HttpMethod,
        path: P,
        headers: Hs,
        body_opt: Option<B>,
    ) -> Result<HttpRequest<HttpBody>, AnyError>
    where
        P: AsRef<str>,
        Hs: IntoIterator<Item = (Hk, Hv)>,
        B: ToXml,
        Hk: AsRef<str>,
        Hv: AsRef<[u8]>,
    {
        let mut builder = HttpRequest::builder().method(method).uri(path.as_ref());

        for (k, v) in headers {
            builder = builder.header(k.as_ref(), v.as_ref());
        }

        let body = if let Some(body) = body_opt {
            let xml = body.to_xml();
            let xml = String::from(&xml);
            HttpBody::from(xml)
        } else {
            HttpBody::empty()
        };

        let request = builder.body(body)?;

        Ok(request)
    }
}
