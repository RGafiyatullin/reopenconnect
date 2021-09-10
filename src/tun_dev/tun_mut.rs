use super::*;

use ::tokio::io::AsyncRead;
use ::tokio::io::AsyncWrite;

impl TunDev {
    pub fn tun_mut<'a>(&'a mut self) -> impl AsyncRead + AsyncWrite + Send + Sync + Unpin + 'a {
        &mut self.tun
    }
}
