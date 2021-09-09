use ::tokio::io::AsyncRead;
use ::tokio::io::AsyncWrite;

pub trait HttpIo: 'static + Send + Sync + AsyncRead + AsyncWrite + Unpin {}

impl<IO> HttpIo for IO
where
    IO: 'static,
    IO: Send + Sync,
    IO: AsyncRead,
    IO: AsyncWrite,
    IO: Unpin,
{
}
