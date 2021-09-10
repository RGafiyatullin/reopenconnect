use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::time::Duration;

use ::ipnetwork::Ipv4Network;

use crate::util::HttpHeaders;

#[derive(Debug, Clone)]
pub struct CstpProps {
    pub address_v4: Ipv4Addr,
    pub netmask_v4: Ipv4Addr,
    pub dns_addrs: Vec<IpAddr>,
    pub idle_timeout: Duration,
    pub disconnected_timeout: Duration,
    pub keepalive: Duration,
    pub split_include: Vec<Ipv4Network>,

    pub http_headers: HttpHeaders,
}
