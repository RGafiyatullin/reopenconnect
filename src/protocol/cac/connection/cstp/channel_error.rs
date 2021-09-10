use crate::AnyError;

#[derive(Debug, ::thiserror::Error)]
pub enum CstpChannelError {
    #[error("CSTP Channel Error: Tun-IO rx")]
    TunIORx(#[source] AnyError),
    #[error("CSTP Channel Error: Tun-IO tx")]
    TunIoTx(#[source] AnyError),

    #[error("CSTP Channel Error: CSTP-IO rx")]
    CstpIORx(#[source] AnyError),
    #[error("CSTP Channel Error: CSTP-IO tx")]
    CstpIoTx(#[source] AnyError),

    #[error("CSTP Channel Error: MPSC")]
    Mpsc(#[source] AnyError),

    #[error("CSTP Unsupported")]
    Unsupported(#[source] AnyError),

    #[error("CSTP Terminate")]
    Terminate,
}
