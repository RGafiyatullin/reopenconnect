#[cfg(feature = "with-tokio-tun")]
use ::tokio_tun::Tun;
#[cfg(feature = "with-null-tun")]
mod null_tun;
#[cfg(feature = "with-null-tun")]
use null_tun::NullTun as Tun;

mod error;
pub use error::TunDevInitError;

mod context;
pub use context::TunDevContext;

mod create;
mod tun_mut;

mod fmt_debug;

pub struct TunDev {
    tun: Tun,
}
