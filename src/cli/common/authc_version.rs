#[derive(Debug, Clone, ::structopt::StructOpt)]
pub struct AuthcVersion {
    #[structopt(long = "set-version")]
    pub ver: String,
}
