use crate::protocol::cac::HttpContext;
use crate::AnyError;

use super::cstp::CstpPhase;
use super::cstp::CstpProps;

#[async_trait::async_trait]
pub trait CstpContext: Send + Sync + HttpContext<CstpPhase> {
    async fn cstp_connected(&mut self, _cstp_props: &CstpProps) -> Result<(), AnyError> {
        Ok(())
    }
    async fn base_mtu(&mut self) -> Result<u16, AnyError> {
        Ok(1500)
    }
    async fn mtu(&mut self) -> Result<u16, AnyError> {
        Ok(1399)
    }
}

#[async_trait::async_trait]
pub trait DtlsContext: Send + Sync {}

impl<T> DtlsContext for T where T: Send + Sync {}
