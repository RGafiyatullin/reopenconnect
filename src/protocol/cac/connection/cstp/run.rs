use super::*;

use std::error::Error as StdError;

use ::futures::channel::mpsc;
use ::futures::prelude::*;
use ::tokio::io::AsyncRead;
use ::tokio::io::AsyncReadExt;
use ::tokio::io::AsyncWrite;
use ::tokio::io::AsyncWriteExt;

use crate::protocol::cac::connection::CstpContext;
use crate::tun_dev::TunDev;
use crate::util::HttpIo;

const MAX_MTU: usize = 64 * 1024;
const CSTP_TX_QUEUE_SIZE: usize = 128;

impl<Ctx> Cstp<Ctx> {
    pub async fn run<IO>(
        &mut self,
        mut cstp_io: IO,
        mut tun_dev: TunDev,
    ) -> Result<(IO, TunDev), CstpChannelError>
    where
        IO: HttpIo,
        Ctx: CstpContext,
    {
        let tun_io = tun_dev.tun_mut();
        let (tun_io_rx, tun_io_tx) = ::tokio::io::split(tun_io);
        let (cstp_io_rx, cstp_io_tx) = ::tokio::io::split(&mut cstp_io);

        let (cstp_q_tx, cstp_q_rx) = mpsc::channel::<CstpFrame>(CSTP_TX_QUEUE_SIZE);

        let tun_dev_read_running = run_tun_dev_read(tun_io_rx, cstp_q_tx.to_owned());
        let cstp_recv_running = run_cstp_recv(cstp_io_rx, tun_io_tx, cstp_q_tx.to_owned());
        let cstp_send_running = run_cstp_send(cstp_q_rx, cstp_io_tx);

        let _ =
            future::try_join3(cstp_send_running, cstp_recv_running, tun_dev_read_running).await?;

        Ok((cstp_io, tun_dev))
    }
}

async fn run_tun_dev_read<TunIoRx>(
    mut tun_io_rx: TunIoRx,
    mut cstp_q_tx: mpsc::Sender<CstpFrame>,
) -> Result<(), CstpChannelError>
where
    TunIoRx: AsyncRead + Send + Sync + Unpin,
    // CstpQTx: Sink<CstpFrame> + Send + Sync + Unpin,
    // CstpQTx::Error: StdError + Send + Sync + 'static,
{
    let mut buf = [0 as u8; MAX_MTU];
    loop {
        let bytes_read = tun_io_rx
            .read(&mut buf[..])
            .await
            .map_err(Into::into)
            .map_err(CstpChannelError::TunIORx)?;
        let ip_packet = &mut buf[0..bytes_read];

        let cstp_frame = CstpFrame::Data(ip_packet[..].to_vec());

        if let Err(reason) = cstp_q_tx.try_send(cstp_frame) {
            if !reason.is_full() {
                Err(CstpChannelError::Mpsc(reason.into()))?
            } else {
                log::warn!("mpsc-full. Dropping a packet");
            }
        }
        // let () = cstp_q_tx
        //     .send(cstp_frame)
        //     .await
        //     .map_err(Into::into)
        //     .map_err(CstpChannelError::Mpsc)?;
    }
}

async fn run_cstp_send<CstpQRx, CstpIOTx>(
    cstp_q_rx: CstpQRx,
    cstp_io_tx: CstpIOTx,
) -> Result<(), CstpChannelError>
where
    CstpQRx: Stream<Item = CstpFrame> + Send + Sync + Unpin,
    CstpIOTx: AsyncWrite + Send + Sync + Unpin,
{
    let _cstp_tx = cstp_q_rx
        .map(Result::<_, CstpChannelError>::Ok)
        .try_fold(cstp_io_tx, |mut cstp_tx, cstp_frame| async move {
            log::trace!("UPSTREAM: {}", cstp_frame);
            let () = cstp_frame
                .write(&mut cstp_tx)
                .await
                .map_err(Into::into)
                .map_err(CstpChannelError::CstpIoTx)?;
            Ok(cstp_tx)
        })
        .await?;

    Ok(())
}

async fn run_cstp_recv<CstpIORx, TunIoTx, CstpQTx>(
    mut cstp_io_rx: CstpIORx,
    mut tun_io_tx: TunIoTx,
    mut cstp_q_tx: CstpQTx,
) -> Result<(), CstpChannelError>
where
    CstpIORx: AsyncRead + Send + Sync + Unpin,
    TunIoTx: AsyncWrite + Send + Sync + Unpin,
    CstpQTx: Sink<CstpFrame> + Send + Sync + Unpin,
    CstpQTx::Error: StdError + Send + Sync + 'static,
{
    loop {
        let cstp_frame = CstpFrame::read(&mut cstp_io_rx)
            .await
            .map_err(Into::into)
            .map_err(CstpChannelError::CstpIORx)?;
        log::trace!("DOWNSTREAM: {}", cstp_frame);

        match cstp_frame {
            CstpFrame::Data(ip_packet) => {
                let () = tun_io_tx
                    .write_all(&ip_packet)
                    .await
                    .map_err(Into::into)
                    .map_err(CstpChannelError::TunIoTx)?;
            }
            CstpFrame::DpdReq(payload) => {
                let () = cstp_q_tx
                    .send(CstpFrame::DpdResp(payload))
                    .await
                    .map_err(Into::into)
                    .map_err(CstpChannelError::Mpsc)?;
            }
            CstpFrame::DpdResp(_) => (),
            CstpFrame::Disconect => Err(CstpChannelError::Terminate)?,
            CstpFrame::Keepalive => cstp_q_tx
                .send(CstpFrame::Keepalive)
                .await
                .map_err(Into::into)
                .map_err(CstpChannelError::Mpsc)?,
            CstpFrame::Terminate => Err(CstpChannelError::Terminate)?,
            CstpFrame::Compressed(_) => Err(::eyre::eyre!("CstpFrame::Compressed — unimplemented"))
                .map_err(CstpChannelError::Unsupported)?,
        }
    }
}
