use crate::AnyError;

#[async_trait::async_trait]
pub trait TunDevContext: Send + Sync + 'static {
    async fn tun_dev_created(&mut self, _tun_dev_name: &str) -> Result<(), AnyError> {
        Ok(())
    }
}
