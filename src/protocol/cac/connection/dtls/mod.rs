use std::net::SocketAddr;
use std::sync::Arc;

use ::webrtc_dtls::cipher_suite::cipher_suite_aes_256_cbc_sha::CipherSuiteAes256CbcSha;
use ::webrtc_dtls::config::Config as DTLSConfig;
use ::webrtc_dtls::conn::DTLSConn;

use ::tokio::net::UdpSocket;

use crate::AnyError;
use crate::AnyhowError;

pub async fn connect(connect_from: SocketAddr, connect_to: SocketAddr) -> Result<(), AnyError> {
    let udp_socket = UdpSocket::bind::<SocketAddr>(connect_from).await?;
    let udp_socket = Arc::new(udp_socket);
    let () = udp_socket.connect(connect_to).await?;

    let dtls_conn = DTLSConn::new(udp_socket, dtls_config(), true, None)
        .await
        .map_err(AnyhowError)?;

    unimplemented!()
}

fn dtls_config() -> DTLSConfig {
    DTLSConfig {
        cipher_suites: vec![],
        ..Default::default()
    }
}
