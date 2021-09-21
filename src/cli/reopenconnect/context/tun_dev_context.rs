use crate::tun_dev::TunDevContext;

use super::*;

#[async_trait::async_trait]
impl TunDevContext for Context {
    async fn tun_dev_created(&mut self, _tun_dev_name: &str) -> Result<(), AnyError> {
        // for network in &self.split_routes_ipv4 {
        //     let mut command = ::tokio::process::Command::new("/sbin/ip");
        //     command
        //         .arg("route")
        //         .arg("add")
        //         .arg(network.to_string())
        //         .arg("dev")
        //         .arg(tun_dev_name)
        //         .arg("proto")
        //         .arg("unspec")
        //         .arg("scope")
        //         .arg("link");
        //     let _ = command.status().await?;
        // }

        Ok(())
    }
}
