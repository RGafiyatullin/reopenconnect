use crate::XmlElement;

#[derive(Debug, Clone, ::structopt::StructOpt)]
pub struct AuthcReply {
    #[structopt(long = "add-authc-reply-node")]
    pub nodes: Vec<XmlElement>,
}
