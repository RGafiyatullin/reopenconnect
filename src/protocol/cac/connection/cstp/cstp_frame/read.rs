use super::*;

use ::tokio::io::AsyncRead;
use ::tokio::io::AsyncReadExt;
use byteorder::ByteOrder;

use crate::protocol::cac::connection::consts::cstp_frame as consts;

impl CstpFrame {
    pub async fn read<IO>(io: &mut IO) -> Result<Self, AnyError>
    where
        IO: AsyncRead + Unpin,
    {
        ::futures::pin_mut!(io);

        let mut header = [0 as u8; 8];
        if io.read_exact(&mut header).await? != header.len() {
            Err(::eyre::eyre!("Failed to read CSTP-header completely"))?
        }
        if &header[0..4] != &consts::MAGIC {
            Err(::eyre::eyre!("Invalid CSTP-header magic"))?
        }

        let payload_len = ::byteorder::BigEndian::read_u16(&header[4..6]) as usize;
        let frame_type = header[6];

        let mut payload = vec![0; payload_len];

        if io.read_exact(&mut payload[..]).await? != payload_len {
            Err(::eyre::eyre!("Failed to read CSTP-body completely"))?
        }

        let frame = match frame_type {
            consts::TYPE_DATA => Self::Data(payload),
            consts::TYPE_DPD_REQ => Self::DpdReq(payload),
            consts::TYPE_DPD_RESP => Self::DpdResp(payload),
            consts::TYPE_DISCONNECT => Self::Disconect,
            consts::TYPE_KEEPALIVE => Self::Keepalive,
            consts::TYPE_COMPRESSED => Self::Compressed(payload),
            consts::TYPE_TERMINATE => Self::Terminate,
            _ => Err(::eyre::eyre!("Invalid CSTP-frame type: {:?}", frame_type))?,
        };

        Ok(frame)
    }
}
