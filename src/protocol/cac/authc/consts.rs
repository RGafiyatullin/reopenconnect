pub const CONFIG_AUTH: &str = "config-auth";

pub mod config_auth_types {
    pub const TYPE_INIT: &str = "init";
    pub const TYPE_AUTH_REQUEST: &str = "auth-request";
    pub const TYPE_AUTH_REPLY: &str = "auth-reply";
    pub const TYPE_COMPLETE: &str = "complete";
    pub const TYPE_LOGOUT: &str = "logout";
}

pub mod config_auth_std {
    pub const DEVICE_ID: &str = "device-id";
    pub const GROUP_ACCESS: &str = "group-access";
    pub const VERSION: &str = "version";
}

pub mod config_auth_opaque {
    pub const OPAQUE: &str = "opaque";
    pub const IS_FOR: &str = "is-for";
}

pub mod config_auth_request {
    pub const AUTH: &str = "auth";
}
pub mod config_auth_complete {
    pub const SESSION_TOKEN: &str = "session-token";
}
