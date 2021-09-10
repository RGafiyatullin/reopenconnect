use std::io::BufRead;

use ::openssl::nid::Nid;
use ::reopenconnect::protocol::cac::strap::Strap;
use ::reopenconnect::AnyError;
use rand::RngCore;

fn main() {
    run().expect("Failure")
}

fn run() -> Result<(), AnyError> {
    let curve = Nid::X9_62_PRIME256V1;
    let digest = Nid::SHA256;

    let mut idx = 0;
    let mut strap = Strap::new(curve)?;
    println!("STRAP[{}]", idx);
    println!("- priv: {}", strap.privkey_b64()?);
    println!("- pub: {}", strap.pubkey_b64()?);

    let mut rng = ::rand::thread_rng();
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut buf = String::new();
    while let Ok(_) = stdin.read_line(&mut buf) {
        idx += 1;
        let mut nonce = vec![0; 12];
        rng.fill_bytes(&mut nonce);
        let mut next = Strap::new(curve)?;
        let () = strap.sign(digest, &nonce[..], &mut next)?;

        println!("STRAP[{}]", idx);
        println!("- priv: {}", next.privkey_b64()?);
        println!("- pub: {}", next.pubkey_b64()?);
        println!("- nonce: {:02x?}", &nonce[..]);
        println!("- verify: {}", next.verify_b64().unwrap());

        strap = next;
    }

    Ok(())
}
