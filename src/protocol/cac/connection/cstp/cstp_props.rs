use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::time::Duration;

use ::ipnetwork::Ipv4Network;

use crate::util::HttpHeaders;

#[derive(Debug, Clone, ::serde::Serialize)]
pub struct CstpProps {
    pub address_v4: Ipv4Addr,
    pub netmask_v4: Ipv4Addr,
    pub dns_addrs: Vec<IpAddr>,
    pub idle_timeout: Duration,
    pub disconnected_timeout: Duration,
    pub keepalive: Duration,
    pub split_include: Vec<Ipv4Network>,

    #[serde(serialize_with = "serialize_http_headers")]
    pub http_headers: HttpHeaders,
}

fn serialize_http_headers<S: ::serde::Serializer>(http_headers: &HttpHeaders, serializer: S) -> Result<S::Ok, S::Error> {
    use ::serde::Serialize;
    
    http_headers.iter().filter_map(|(header_name, header_value)| {
        let header_name = header_name.as_str();
        if let Ok(header_value) = std::str::from_utf8(header_value.as_bytes()) {
            Some((header_name, header_value))
        } else {
            None
        }
    }).collect::<Vec<_>>().serialize(serializer)
}


