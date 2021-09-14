use ::openssl::nid::Nid;
use ::rand::RngCore;

use crate::protocol::cac::strap::Strap;
use crate::AnyError;

#[derive(Debug, Clone, ::structopt::StructOpt)]
pub struct ReOpenConnectStrap {
    #[structopt(subcommand)]
    sub: Sub,
}

#[derive(Debug, Clone, ::structopt::StructOpt)]
pub enum Sub {
    New,
    Next { prev: String },
}

impl ReOpenConnectStrap {
    pub fn run(&self) -> Result<(), AnyError> {
        let curve = Nid::X9_62_PRIME256V1;
        let digest = Nid::SHA256;

        let mut strap = Strap::new(curve)?;

        match &self.sub {
            Sub::New => (),
            Sub::Next { prev } => {
                let mut rng = ::rand::thread_rng();
                let mut nonce = vec![0; 12];
                rng.fill_bytes(&mut nonce);
                let prev = Strap::from_privkey_b64(&prev)?;
                prev.sign(digest, &nonce[..], &mut strap)?;
            }
        }

        println!("privkey: {}", strap.privkey_b64()?);
        println!("pubkey: {}", strap.pubkey_b64()?);

        if let Some(verify) = strap.verify_b64() {
            println!("verify: {}", verify);
        }

        Ok(())
    }
}
