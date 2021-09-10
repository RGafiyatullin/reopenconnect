#[derive(Debug, Clone, ::structopt::StructOpt)]
pub struct TunDevArgs {
    #[structopt(long)]
    pub tun_dev_name: String,
}
