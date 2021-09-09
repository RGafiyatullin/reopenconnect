use std::borrow::BorrowMut;

use super::request;
use super::AuthenticationPhase;

#[async_trait::async_trait]
pub trait AuthenticatorContext: Send + Sync + AuthenticatorContextInit {
    async fn user_agent(&mut self) -> String {
        str!("reconnect")
    }
    async fn http_host(&mut self) -> String;

    async fn patch_http_headers(
        &mut self,
        _phase: AuthenticationPhase,
        headers: Vec<(String, String)>,
    ) -> Vec<(String, String)> {
        headers
    }
}

#[async_trait::async_trait]
pub trait AuthenticatorContextInit: Send + Sync {
    async fn version(&mut self) -> request::init::Version {
        request::init::Version::new("0.1")
    }
    async fn device_id(&mut self) -> request::init::DeviceID {
        request::init::DeviceID::new("turing-machine")
    }
    async fn group_access(&mut self) -> request::init::GroupAccess;
}

pub trait BorrowMutAuthenticatorContext: BorrowMut<dyn AuthenticatorContext> {}
impl<T> BorrowMutAuthenticatorContext for T where T: BorrowMut<dyn AuthenticatorContext> {}
