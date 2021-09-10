use ::futures::lock::Mutex;

use crate::protocol::cac::connection::cstp::CstpProps;
use crate::tun_dev::TunDev;
use crate::tun_dev::TunDevContext;

use super::*;

pub async fn run<Ctx>(
    tun_dev_args: &TunDevArgs,
    cstp_props: &CstpProps,
    context: &Mutex<Ctx>,
) -> Result<TunDev, AnyError>
where
    Ctx: TunDevContext,
{
    let tun_dev_name = tun_dev_args.tun_dev_name.as_str();

    let tun_dev = TunDev::create(
        &tun_dev_name,
        cstp_props.address_v4,
        cstp_props.netmask_v4,
        context,
    )
    .await?;

    // {
    //     // TODO: Split it into some kind of TunDevContext

    //     let mut resolv_conf = ::tokio::fs::OpenOptions::new()
    //         .truncate(true)
    //         .write(true)
    //         .open("/etc/resolv.conf")
    //         .await?;
    //     for ns_server in &cstp_props.dns_addrs {
    //         let () = resolv_conf
    //             .write_all(format!("nameserver\t{}\n", ns_server).as_bytes())
    //             .await?;
    //     }
    //     let () = resolv_conf.flush().await?;
    // }

    Ok(tun_dev)
}
