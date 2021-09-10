use ::reopenconnect::AnyError;
use webrtc_dtls::cipher_suite::CipherSuite;

#[tokio::main]
async fn main() {
    run().await.expect("Failure")
}

async fn run() -> Result<(), AnyError> {
    use ::webrtc_dtls::cipher_suite::cipher_suite_aes_256_cbc_sha::CipherSuiteAes256CbcSha;

    let suite = CipherSuiteAes256CbcSha::new(false);
    let suite_id = suite.id();

    println!("SUITE-ID: {:?}", suite_id);

    unimplemented!()
}
