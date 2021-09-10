use super::CliKV;

#[derive(Debug, Clone, ::structopt::StructOpt)]
pub struct AddHttpHeadersSimple {
    #[structopt(long = "add-http-header")]
    pub headers: Vec<CliKV>,
}

#[derive(Debug, Clone, ::structopt::StructOpt)]
pub struct AddHttpHeadersForPhases {
    #[structopt(long = "add-http-header-authenticate")]
    pub authenticate_headers: Vec<CliKV>,
    #[structopt(long = "add-http-header-connect")]
    pub connect_headers: Vec<CliKV>,
}
