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
    Next {
        prev: String,
    },
    Debug {
        prev: String,
        next: String,
        nonce: String,
    },
}

impl ReOpenConnectStrap {
    pub fn run(&self) -> Result<(), AnyError> {
        let () = pretty_env_logger::init_timed();

        let curve = Nid::X9_62_PRIME256V1;
        let digest = Nid::SHA256;

        let mut strap = match &self.sub {
            Sub::Debug { next, .. } => Strap::from_privkey_b64(&next)?,
            _ => Strap::new(curve)?,
        };

        log::trace!("strap.pubkey: {:?}", strap.pubkey_b64()?);
        log::trace!("strap.privkey: {:?}", strap.privkey_b64()?);

        match &self.sub {
            Sub::New => (),
            Sub::Next { prev } => {
                let mut rng = ::rand::thread_rng();
                let mut nonce = vec![0; 12];
                rng.fill_bytes(&mut nonce);
                let prev = Strap::from_privkey_b64(&prev)?;
                prev.sign(digest, &nonce[..], &mut strap)?;
            }
            Sub::Debug { prev, nonce, .. } => {
                let prev = Strap::from_privkey_b64(&prev)?;
                let nonce = ::hex::decode(&nonce)?;

                log::trace!("prev.pubkey: {:?}", prev.pubkey_b64()?);
                log::trace!("prev.privkey: {:?}", prev.privkey_b64()?);

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
