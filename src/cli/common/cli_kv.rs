use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct CliKV {
    pub key: String,
    pub value: String,
}
impl FromStr for CliKV {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (key, value) = s.split_once(':').ok_or_else(|| {
            format!(
                "{} should be in format <key>:<value>",
                std::any::type_name::<Self>()
            )
        })?;
        let added_header = Self {
            key: key.to_owned(),
            value: value.to_owned(),
        };
        Ok(added_header)
    }
}
