use super::*;

use std::fmt;

impl<Ctx> fmt::Debug for Cstp<Ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("cstp_props", &self.cstp_props)
            .finish()
    }
}
