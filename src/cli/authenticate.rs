use super::common::*;

#[derive(Debug, Clone, ::structopt::StructOpt)]
pub struct Authenticate {
    #[structopt(flatten)]
    pub add_http_headers: AddHttpHeadersSimple,

    #[structopt(flatten)]
    pub authc_device_id: AuthcDeviceId,

    #[structopt(flatten)]
    pub authc_version: AuthcVersion,

    #[structopt(flatten)]
    pub authc_rq: AuthcRq,

    #[structopt(flatten)]
    pub authc_reply: AuthcReply,
}
