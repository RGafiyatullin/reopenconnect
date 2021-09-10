use super::*;

use std::fs::OpenOptions;
use std::sync::Arc;

use ::rustls::ClientConfig as TlsClientConfig;
use ::rustls::RootCertStore;
use ::tokio_rustls::TlsConnector;

pub async fn run(cert_store_args: &CertStoreArgs) -> Result<TlsConnector, AnyError> {
    let mut root_cert_store = if cert_store_args.dont_use_native_cert_store {
        RootCertStore::empty()
    } else {
        match rustls_native_certs::load_native_certs() {
            Ok(cert_store) => cert_store,
            Err((Some(cert_store), _)) => cert_store,
            Err((None, err)) => Err(err)?,
        }
    };
    for ca_roots_file in &cert_store_args.add_ca_certs {
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
