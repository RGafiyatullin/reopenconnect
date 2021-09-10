use super::*;

impl ReOpenConnect {
    pub(super) fn http_headers(&self) -> &[CliKV] {
        &self.add_http_headers.headers
    }
    pub(super) fn http_headers_for_authc(&self) -> &[CliKV] {
        match &self.sub {
            Sub::AuthenticateAndConnect(sub) => &sub.http_headers.authenticate_headers,
            Sub::AuthenticateOnly(sub) => &sub.add_http_headers.headers,
            _ => &[],
        }
    }
    pub(super) fn http_headers_for_connect(&self) -> &[CliKV] {
        match &self.sub {
            Sub::AuthenticateAndConnect(sub) => &sub.http_headers.connect_headers,
            Sub::ConnectOnly(sub) => &sub.add_http_headers.headers,
            _ => &[],
        }
    }

    pub(super) fn authc_device_id(&self) -> Option<&AuthcDeviceId> {
        match &self.sub {
            Sub::AuthenticateAndConnect(sub) => Some(&sub.authc_device_id),
            Sub::AuthenticateOnly(sub) => Some(&sub.authc_device_id),
            _ => None,
        }
    }
    pub(super) fn authc_rq(&self) -> Option<&AuthcRq> {
        match &self.sub {
            Sub::AuthenticateAndConnect(sub) => Some(&sub.authc_rq),
            Sub::AuthenticateOnly(sub) => Some(&sub.authc_rq),
            _ => None,
        }
    }
    pub(super) fn authc_version(&self) -> Option<&AuthcVersion> {
        match &self.sub {
            Sub::AuthenticateAndConnect(sub) => Some(&sub.authc_version),
            Sub::AuthenticateOnly(sub) => Some(&sub.authc_version),
            _ => None,
        }
    }
    pub(super) fn authc_reply(&self) -> Option<&AuthcReply> {
        match &self.sub {
            Sub::AuthenticateAndConnect(sub) => Some(&sub.authc_reply),
            Sub::AuthenticateOnly(sub) => Some(&sub.authc_reply),
            _ => None,
        }
    }
}
