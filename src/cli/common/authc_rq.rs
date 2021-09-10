use super::CliKV;

use crate::XmlElement;

#[derive(Debug, Clone, ::structopt::StructOpt)]
pub struct AuthcRq {
    #[structopt(long = "add-rq-attr")]
    pub add_rq_attr: Vec<CliKV>,

    #[structopt(long = "add-rq-child")]
    pub add_rq_child: Vec<XmlElement>,
}
