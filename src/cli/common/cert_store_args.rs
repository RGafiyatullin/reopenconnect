#[derive(Debug, Clone, ::structopt::StructOpt)]
pub struct CertStoreArgs {
    #[structopt(long)]
    pub dont_use_native_cert_store: bool,

    #[structopt(long)]
    pub add_ca_certs: Vec<String>,
}
