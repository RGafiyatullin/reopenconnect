use super::*;

use std::fmt;

impl fmt::Debug for TunDev {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(feature = "with-tokio-tun")]
        let ret = f
            .debug_struct(std::any::type_name::<Self>())
            .field("tun_dev", &self.tun.name())
            .field("dev_addr", &self.tun.address())
            .field("netmask", &self.tun.netmask())
            .field("peer_addr", &self.tun.destination())
            .finish();

        #[cfg(feature = "with-null-tun")]
        let ret = f.debug_struct(std::any::type_name::<Self>()).finish();

        ret
    }
}
