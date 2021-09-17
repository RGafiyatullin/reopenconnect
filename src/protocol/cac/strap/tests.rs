use super::*;

#[test]
fn t01() {
    let strap0 = Strap::new(OpenSslNid::X9_62_PRIME256V1).expect("Failed to create STRAP");
    println!("STRAP[0].pub-key: {}", strap0.pubkey_b64().unwrap());
    println!("STRAP[0].priv-key: {}", strap0.privkey_b64().unwrap());

    let mut strap1 = Strap::new(OpenSslNid::X9_62_PRIME256V1).expect("Failed to create STRAP");
    strap0
        .sign(
            OpenSslNid::SHA256,
            &[
                0x48, 0x81, 0x34, 0xe3, 0xec, 0x61, 0x62, 0xc2, 0x2d, 0x54, 0xf5, 0xa7,
            ],
            &mut strap1,
        )
        .expect("Failed to rekey");
    println!("STRAP[1].pub-key: {}", strap1.pubkey_b64().unwrap());
    println!("STRAP[1].priv-key: {}", strap1.privkey_b64().unwrap());
    println!("STRAP[1].verify: {}", strap1.verify_b64().unwrap());
}

#[test]
fn t02() {
    let strap0 = Strap::from_privkey_b64("MHcCAQEEIG1GFAQcDFoW5ad/7pvBEtF20ATV7mbkJTWAn5iKE0j9oAoGCCqGSM49AwEHoUQDQgAEk/45fPmmDpMW83+DfbXOjIfrjeabOHjn5FAOYUI/Rt0ugm535gSmmZnCO+WuCwlN28bt2Eulun9dJ24QnOKUYg==").expect("Failed to restore STRAP");

    assert_eq!(strap0.privkey_b64().unwrap(), str!("MHcCAQEEIG1GFAQcDFoW5ad/7pvBEtF20ATV7mbkJTWAn5iKE0j9oAoGCCqGSM49AwEHoUQDQgAEk/45fPmmDpMW83+DfbXOjIfrjeabOHjn5FAOYUI/Rt0ugm535gSmmZnCO+WuCwlN28bt2Eulun9dJ24QnOKUYg=="));
    assert_eq!(strap0.pubkey_b64().unwrap(), str!("MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEk/45fPmmDpMW83+DfbXOjIfrjeabOHjn5FAOYUI/Rt0ugm535gSmmZnCO+WuCwlN28bt2Eulun9dJ24QnOKUYg=="));

    let mut strap1 = Strap::from_privkey_b64("MHcCAQEEID/woKsyxjzTWLJbYOOXg7SHwPHSkrMWAsEW5RfhviCUoAoGCCqGSM49AwEHoUQDQgAET3g1myvtTlxpn46WIAcnuuRlQSmn2bUCEXDqhDbAshJuQXh8hlcxikrvFQB5XY2nonoLKjDzxNWRAz/iSwDbjQ==").expect("Failed to restore STRAP");

    assert_eq!(strap1.privkey_b64().unwrap(), str!("MHcCAQEEID/woKsyxjzTWLJbYOOXg7SHwPHSkrMWAsEW5RfhviCUoAoGCCqGSM49AwEHoUQDQgAET3g1myvtTlxpn46WIAcnuuRlQSmn2bUCEXDqhDbAshJuQXh8hlcxikrvFQB5XY2nonoLKjDzxNWRAz/iSwDbjQ=="));
    assert_eq!(strap1.pubkey_b64().unwrap(), str!("MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAET3g1myvtTlxpn46WIAcnuuRlQSmn2bUCEXDqhDbAshJuQXh8hlcxikrvFQB5XY2nonoLKjDzxNWRAz/iSwDbjQ=="));

    strap0
        .sign(
            OpenSslNid::SHA256,
            &[
                0xf7, 0xf1, 0x7d, 0xe4, 0xe9, 0xcc, 0xa5, 0xe9, 0x19, 0x60, 0x78, 0xf3,
            ],
            &mut strap1,
        )
        .expect("Failed to sign");

    // assert_eq!(strap1.verify_b64(), Some(str!("")));
}

#[test]
fn f03() {
    let strap = Strap::from_privkey_b64("MHcCAQEEIJV55XPXEUczqxJ+7XVaCVxHczD8iu7exfFimpuT5ZTIoAoGCCqGSM49AwEHoUQDQgAEoXWlyxI++eD+lhLKXCfH8dIzPxF175I2otM+WzFLN1zC9IxwWwalhaTL21+AAJx4HH2eBUs3Vz0qxU16f3QyxQ==").unwrap();
    assert_eq!(strap.pubkey_b64().unwrap(), "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEoXWlyxI++eD+lhLKXCfH8dIzPxF175I2otM+WzFLN1zC9IxwWwalhaTL21+AAJx4HH2eBUs3Vz0qxU16f3QyxQ==");
}
