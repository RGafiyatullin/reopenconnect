use std::sync::Arc;

use ::futures::lock::Mutex;

use crate::protocol::cac::connection::Cstp;

use super::*;

pub async fn run<IO>(
    io: IO,
    session_token: &str,
    context: Context,
) -> Result<(IO, Cstp<Context>), AnyError>
where
    IO: HttpIo,
{
    let context = Arc::new(Mutex::new(context));
    let (cstp, io) = Cstp::connect(io, session_token, context).await?;
    Ok((io, cstp))
}
