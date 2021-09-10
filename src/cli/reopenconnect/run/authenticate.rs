use crate::protocol::cac::authc::Authenticator;
use crate::protocol::cac::authc::AuthenticatorContext;

use super::*;

pub async fn run<IO>(io: IO, context: &mut Context) -> Result<(IO, String), AnyError>
where
    IO: HttpIo,
{
    let authc_ctx: &mut dyn AuthenticatorContext = context;
    let mut authenticator = Authenticator::create(io, authc_ctx).await?;
    let complete = authenticator.run().await?;
    let io = authenticator.into_io().await?;
    Ok((io, complete.session_token))
}
