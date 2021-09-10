use super::*;

use crate::protocol::cac::connection::cstp::CstpProps;
use crate::protocol::cac::connection::CstpContext;

#[async_trait::async_trait]
impl CstpContext for Context {
    async fn cstp_connected(&mut self, cstp_props: &CstpProps) -> Result<(), AnyError> {
        self.split_routes_ipv4 = cstp_props.split_include.to_owned();
        Ok(())
    }
}
