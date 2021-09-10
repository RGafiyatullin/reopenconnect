use crate::util::HttpHeaders;
use crate::AnyError;

#[async_trait::async_trait]
pub trait HttpContext<Phase>: Send + Sync
where
    Phase: Copy + Send + Sync + 'static,
{
    async fn http_host(&mut self) -> Result<String, AnyError>;

    async fn patch_http_headers(
        &mut self,
        _phase: Phase,
        _headers: &mut HttpHeaders,
    ) -> Result<(), AnyError> {
        Ok(())
    }
}
