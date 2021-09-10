use super::*;

use ::hyper::header::HeaderName;
use ::hyper::header::HeaderValue;

use crate::protocol::cac::authc::AuthenticationPhase;
use crate::protocol::cac::connection::cstp::CstpPhase;
use crate::protocol::cac::HttpContext;
use crate::util::HttpHeaders;

#[async_trait::async_trait]
impl HttpContext<()> for Context {
    async fn http_host(&mut self) -> Result<String, AnyError> {
        Ok(self.args.connect_to.http_host()?.to_owned())
    }
    async fn patch_http_headers(
        &mut self,
        _phase: (),
        headers: &mut HttpHeaders,
    ) -> Result<(), AnyError> {
        for add_header in self.args.http_headers() {
            let _ = headers.append(
                add_header.key.parse::<HeaderName>()?,
                HeaderValue::from_str(add_header.value.to_owned().as_str())?,
            );
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl HttpContext<CstpPhase> for Context {
    async fn http_host(&mut self) -> Result<String, AnyError> {
        HttpContext::<()>::http_host(self).await
    }
    async fn patch_http_headers(
        &mut self,
        _phase: CstpPhase,
        headers: &mut HttpHeaders,
    ) -> Result<(), AnyError> {
        let () = HttpContext::<()>::patch_http_headers(self, (), headers).await?;

        for add_header in self.args.http_headers_for_connect() {
            let _ = headers.append(
                add_header.key.parse::<HeaderName>()?,
                HeaderValue::from_str(add_header.value.to_owned().as_str())?,
            );
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl HttpContext<AuthenticationPhase> for Context {
    async fn http_host(&mut self) -> Result<String, AnyError> {
        HttpContext::<()>::http_host(self).await
    }
    async fn patch_http_headers(
        &mut self,
        _phase: AuthenticationPhase,
        headers: &mut HttpHeaders,
    ) -> Result<(), AnyError> {
        let () = HttpContext::<()>::patch_http_headers(self, (), headers).await?;

        for add_header in self.args.http_headers_for_authc() {
            let _ = headers.append(
                add_header.key.parse::<HeaderName>()?,
                HeaderValue::from_str(add_header.value.to_owned().as_str())?,
            );
        }

        Ok(())
    }
}
