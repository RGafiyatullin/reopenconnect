use std::fs::OpenOptions;

use std::sync::Arc;

use ::rustls::ClientConfig as TlsClientConfig;
use ::rustls::RootCertStore;
use ::tokio_rustls::TlsConnector;

use crate::AnyError;

#[derive(Debug, Clone, ::structopt::StructOpt)]
pub struct CARootsConfig {
    #[structopt(long = "add-ca-roots")]
    pub files: Vec<String>,
}

impl CARootsConfig {
    pub fn create_tls_connector(&self) -> Result<TlsConnector, AnyError> {
        let mut root_cert_store = RootCertStore::empty();
        for ca_roots_file in &self.files {
            let ca_roots_fd = OpenOptions::new().read(true).open(ca_roots_file)?;
            let (roots_added, roots_ignored) = root_cert_store
                .add_pem_file(&mut std::io::BufReader::new(ca_roots_fd))
                .expect("Failed to add CA-roots");
            log::debug!(
                "CA-roots [added: {}; ignored: {}]",
                roots_added,
                roots_ignored
            );
        }

        let mut client_config = TlsClientConfig::default();
        client_config.root_store = root_cert_store;
        let client_config = Arc::new(client_config);
        let tls_connector = TlsConnector::from(client_config);

        Ok(tls_connector)
    }
}
