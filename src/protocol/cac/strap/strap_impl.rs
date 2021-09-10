use super::*;

use ::openssl::ec::EcGroup;
use ::openssl::hash::MessageDigest;
use ::openssl::nid::Nid;
use ::openssl::pkey::PKey;
use ::openssl::sign::Signer;

impl Strap {
    pub fn new(curve_nid: Nid) -> Result<Self, StrapError> {
        let ec_group = EcGroup::from_curve_name(curve_nid)?;
        let ec_key = EcKey::generate(&ec_group)?;
        let strap = Self {
            ec_key,
            verify: None,
        };

        Ok(strap)
    }
    pub fn from_privkey_der(der: &[u8]) -> Result<Self, StrapError> {
        let ec_key = EcKey::private_key_from_der(der)?;

        let ret = Self {
            ec_key,
            verify: None,
        };

        Ok(ret)
    }
    pub fn from_privkey_b64(b64: &str) -> Result<Self, StrapError> {
        let der = ::base64::decode(b64)?;
        Self::from_privkey_der(&der)
    }

    pub fn pubkey_der(&self) -> Result<Vec<u8>, StrapError> {
        let pubkey_der = self.ec_key.public_key_to_der()?;
        Ok(pubkey_der)
    }
    pub fn pubkey_b64(&self) -> Result<String, StrapError> {
        self.pubkey_der().map(::base64::encode)
    }

    pub fn privkey_der(&self) -> Result<Vec<u8>, StrapError> {
        let privkey_der = self.ec_key.private_key_to_der()?;
        Ok(privkey_der)
    }
    pub fn privkey_b64(&self) -> Result<String, StrapError> {
        self.privkey_der().map(::base64::encode)
    }

    pub fn verify(&self) -> Option<&[u8]> {
        self.verify.as_ref().map(AsRef::as_ref)
    }
    pub fn verify_b64(&self) -> Option<String> {
        self.verify().map(::base64::encode)
    }

    pub fn sign(&self, md_nid: Nid, nonce: &[u8], next: &mut Strap) -> Result<(), StrapError> {
        let message_digest = MessageDigest::from_nid(md_nid)
            .ok_or_else(|| StrapError::BadMessageDigestNid(md_nid))?;
        let pkey = PKey::from_ec_key(self.ec_key.to_owned())?;
        let mut signer = Signer::new(message_digest, &pkey)?;
        let () = signer.update(nonce)?;
        let () = signer.update(&next.pubkey_der()?)?;
        let verify = signer.sign_to_vec()?;
        next.verify = Some(verify);

        Ok(())
    }
}
