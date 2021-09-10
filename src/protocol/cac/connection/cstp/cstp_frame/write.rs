use super::*;

use ::tokio::io::AsyncWrite;
use ::tokio::io::AsyncWriteExt;
use ::tokio::io::BufWriter;

use crate::protocol::cac::connection::consts::cstp_frame as consts;

impl CstpFrame {
    pub async fn write<IO>(&self, io: &mut IO) -> Result<(), AnyError>
    where
        IO: AsyncWrite + Unpin,
    {
        let io = BufWriter::new(io);
        ::futures::pin_mut!(io);

        let (frame_type, payload) = match self {
            Self::Data(payload) => (consts::TYPE_DATA, &payload[..]),
            Self::DpdReq(payload) => (consts::TYPE_DPD_REQ, &payload[..]),
            Self::DpdResp(payload) => (consts::TYPE_DPD_RESP, &payload[..]),
            Self::Disconect => (consts::TYPE_DISCONNECT, &[][..]),
            Self::Keepalive => (consts::TYPE_KEEPALIVE, &[][..]),
            Self::Compressed(payload) => (consts::TYPE_COMPRESSED, &payload[..]),
            Self::Terminate => (consts::TYPE_TERMINATE, &[][..]),
        };

        let () = io.write_all(&consts::MAGIC).await?;
        let () = io.write_u16(payload.len() as u16).await?;
        let () = io.write_u8(frame_type).await?;
        let () = io.write_u8(0x00).await?;

        let () = io.write_all(payload).await?;

        let () = io.flush().await?;

        Ok(())
    }
}
