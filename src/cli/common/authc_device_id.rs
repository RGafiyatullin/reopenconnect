use super::CliKV;

#[derive(Debug, Clone, ::structopt::StructOpt)]
pub struct AuthcDeviceId {
    #[structopt(long = "add-device-id-attr")]
    pub add_device_id_attr: Vec<CliKV>,

    #[structopt(long = "set-device-id-platform")]
    pub device_id_platform: Option<String>,
}
