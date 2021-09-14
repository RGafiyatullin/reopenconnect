use super::common::*;

#[derive(Debug, Clone, ::structopt::StructOpt)]
pub struct Connect {
    #[structopt(flatten)]
    pub add_http_headers: AddHttpHeadersSimple,

    #[structopt(long)]
    pub session_token: String,

    #[structopt(flatten)]
    pub tun_dev: TunDevArgs,

    #[structopt(long)]
    pub on_connected: Option<String>,
}
