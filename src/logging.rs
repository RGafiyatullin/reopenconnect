#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoggingBackend {
    Syslog,
    Stderr,
}

#[derive(Debug, ::structopt::StructOpt)]
pub struct LoggingConfig {
    #[structopt(long = "log-backend", default_value = "stderr")]
    pub backend: LoggingBackend,
}

impl std::str::FromStr for LoggingBackend {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = match s {
            "syslog" => Self::Syslog,
            "stderr" => Self::Stderr,
            unknown => Err(format!("Unknown LoggingBackend: {:?}", unknown))?,
        };
        Ok(v)
    }
}
