use super::*;

use std::collections::HashMap;

use ::ipnetwork::Ipv4Network;

use crate::XmlElement;

mod authc_context;
mod cstp_context;
mod http_context;
mod tun_dev_context;

#[derive(Debug, Clone)]
pub struct Context {
    args: ReOpenConnect,
    opaques: HashMap<String, XmlElement>,
    split_routes_ipv4: Vec<Ipv4Network>,
}

impl Context {
    pub fn new(args: &ReOpenConnect) -> Result<Self, AnyError> {
        Ok(Self {
            args: args.to_owned(),
            opaques: Default::default(),
            split_routes_ipv4: Default::default(),
        })
    }
}
