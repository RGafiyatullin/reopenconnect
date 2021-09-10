use super::*;

use std::fmt;

impl fmt::Display for CstpFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (cstp_type, cstp_payload) = match self {
            Self::Data(payload) => ("DATA", &payload[..]),
            Self::DpdReq(payload) => ("DPD-REQ", &payload[..]),
            Self::DpdResp(payload) => ("DPD-RESP", &payload[..]),
            Self::Disconect => ("DISCONNECT", &[][..]),
            Self::Keepalive => ("KEEPALIVE", &[][..]),
            Self::Compressed(payload) => ("COMPRESSED", &payload[..]),
            Self::Terminate => ("TERMINATE", &[][..]),
        };

        let () = write!(f, "CSTP:{}", cstp_type)?;
        let () = ::pretty_hex::pretty_hex_write(f, &cstp_payload)?;

        Ok(())
    }
}
