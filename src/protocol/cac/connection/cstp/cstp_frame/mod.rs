use crate::AnyError;

mod read;
mod write;

mod fmt_display;

#[derive(Debug)]
pub enum CstpFrame {
    Data(Vec<u8>),
    DpdReq(Vec<u8>),
    DpdResp(Vec<u8>),
    Disconect,
    Keepalive,
    Compressed(Vec<u8>),
    Terminate,
}
