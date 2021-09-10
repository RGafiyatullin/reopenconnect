use std::sync::Arc;

use ::futures::lock::Mutex;

use super::consts;

mod cstp_props;
pub use cstp_props::CstpProps;

mod cstp_frame;
pub use cstp_frame::CstpFrame;

mod connect_error;
pub use connect_error::CstpConnectError;

mod channel_error;
pub use channel_error::CstpChannelError;

mod connect;
mod run;

mod fmt_debug;
mod on_connected;

#[derive(Debug, Clone, Copy)]
pub struct CstpPhase;

pub struct Cstp<Ctx> {
    pub context: Arc<Mutex<Ctx>>,
    pub cstp_props: CstpProps,
}
