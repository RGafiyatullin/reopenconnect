use super::common::*;

#[derive(Debug, Clone, ::structopt::StructOpt)]
pub struct AuthenticateAndConnect {
    #[structopt(flatten)]
    pub http_headers: AddHttpHeadersForPhases,

    #[structopt(flatten)]
    pub authc_device_id: AuthcDeviceId,

    #[structopt(flatten)]
    pub authc_version: AuthcVersion,

    #[structopt(flatten)]
    pub authc_rq: AuthcRq,

    #[structopt(flatten)]
    pub authc_reply: AuthcReply,

    #[structopt(long)]
    pub restart_io_to_connect: bool,

    #[structopt(flatten)]
    pub tun_dev: TunDevArgs,
}
