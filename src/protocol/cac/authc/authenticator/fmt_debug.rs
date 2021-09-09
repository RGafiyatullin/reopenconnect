use super::*;

use std::fmt;

impl<IO, Ctx> fmt::Debug for Authenticator<IO, Ctx>
where
    IO: HttpIo,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(std::any::type_name::<Self>()).finish()
    }
}
