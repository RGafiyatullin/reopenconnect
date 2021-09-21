use std::net::SocketAddr;
use std::sync::Arc;

use ::reopenconnect::AnyError;
use ::reopenconnect::AnyhowError;

use ::tokio::net::UdpSocket;
use ::webrtc_dtls::cipher_suite::cipher_suite_aes_256_cbc_sha::CipherSuiteAes256CbcSha;
use ::webrtc_dtls::cipher_suite::CipherSuite;
use ::webrtc_dtls::config::Config as DtlsConfig;
use ::webrtc_dtls::conn::DTLSConn;

#[tokio::main]
async fn main() {
    run(::structopt::StructOpt::from_args())
        .await
        .expect("Failure")
}

#[derive(Debug, ::structopt::StructOpt)]
struct Args {
    #[structopt(long)]
    remote_addr: SocketAddr,

    #[structopt(long)]
    mtu: usize,

    #[structopt(long)]
    master_secret: String,

    #[structopt(long)]
    session_id: String,
}

async fn run(args: Args) -> Result<(), AnyError> {
    let master_secret = ::hex::decode(&args.master_secret)?;
    let client_random = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1];
    let server_random = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1];

    let dtls_config = DtlsConfig {
        mtu: args.mtu,
        ..DtlsConfig::default()
    };

    let mut suite = CipherSuiteAes256CbcSha::new(true);
    let () = suite
        .init(&master_secret, client_random, server_random, true)
        .map_err(AnyhowError)?;

    let udp_sock = UdpSocket::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap()).await?;
    let () = udp_sock.connect(args.remote_addr).await?;
    let udp_sock = Arc::new(udp_sock);

    let _dtls_conn = DTLSConn::new(udp_sock, dtls_config, true, None)
        .await
        .map_err(AnyhowError)?;

    unimplemented!()
}
