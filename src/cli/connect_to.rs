#[derive(Debug, Clone, ::structopt::StructOpt)]
pub struct ConnectTo {
    #[structopt(long)]
    pub address: String,
    #[structopt(long)]
    pub host: String,
}
