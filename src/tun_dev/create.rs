use std::net::Ipv4Addr;

use ::futures::lock::Mutex;

#[cfg(feature = "with-tokio-tun")]
use tokio_tun::TunBuilder;

use super::*;
use crate::AnyError;

impl TunDev {
    pub async fn create<Ctx>(
        tun_dev_name: &str,
        dev_addr: Ipv4Addr,
        netmask: Ipv4Addr,
        context: &Mutex<Ctx>,
    ) -> Result<Self, AnyError>
    where
        Ctx: TunDevContext,
    {
        #[cfg(feature = "with-tokio-tun")]
        let tun = TunBuilder::new()
            .name(tun_dev_name)
            .packet_info(false)
            .address(dev_addr)
            .netmask(netmask)
            .up()
            .try_build()
            .map_err(TunDevInitError)?;
        #[cfg(feature = "with-null-tun")]
        let tun = Tun;

        {
            let mut ctx = context.lock().await;
            let () = ctx.tun_dev_created(tun_dev_name).await?;
        }

        let tun_dev = Self { tun };
        Ok(tun_dev)
    }
}
