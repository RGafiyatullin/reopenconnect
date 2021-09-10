use std::borrow::BorrowMut;

use super::AuthenticationPhase;
use crate::protocol::cac::authc::client_pdu;
use crate::protocol::cac::authc::server_pdu;
use crate::protocol::cac::HttpContext;
use crate::util::HttpHeaders;
use crate::AnyError;

#[async_trait::async_trait]
pub trait AuthenticatorContext:
    Send
    + Sync
    + HttpContext<AuthenticationPhase>
    + AuthenticatorContextInit
    + AuthenticatorContextRequest
    + AuthenticatorContextComplete
    + AuthenticatorContextReply
{
    async fn patch_version(
        &mut self,
        _phase: AuthenticationPhase,
        _version: &mut client_pdu::common::Version,
    ) -> Result<(), AnyError> {
        Ok(())
    }
    async fn patch_device_id(
        &mut self,
        _phase: AuthenticationPhase,
        _device_id: &mut client_pdu::common::DeviceID,
    ) -> Result<(), AnyError> {
        Ok(())
    }
    async fn group_access(
        &mut self,
        _phase: AuthenticationPhase,
        _group_access: &mut client_pdu::common::GroupAccess,
    ) -> Result<(), AnyError> {
        Ok(())
    }
}

#[async_trait::async_trait]
pub trait AuthenticatorContextInit: Send + Sync {
    async fn patch_init_rq(
        &mut self,
        _init_rq: &mut client_pdu::AuthConfigInit,
    ) -> Result<(), AnyError> {
        Ok(())
    }
}

#[async_trait::async_trait]
pub trait AuthenticatorContextRequest: Send + Sync {
    async fn received_request(
        &mut self,
        _http_headers: &HttpHeaders,
        _req: &server_pdu::AuthConfigRequest,
    ) -> Result<(), AnyError> {
        Ok(())
    }
}

#[async_trait::async_trait]
pub trait AuthenticatorContextReply: Send + Sync {
    async fn patch_reply_rq(
        &mut self,
        _reply_rq: &mut client_pdu::AuthConfigReply,
    ) -> Result<(), AnyError> {
        Ok(())
    }
}

#[async_trait::async_trait]
pub trait AuthenticatorContextComplete: Send + Sync {
    async fn received_complete(
        &mut self,
        _http_headers: &HttpHeaders,
        _req: &server_pdu::AuthConfigComplete,
    ) -> Result<(), AnyError> {
        Ok(())
    }
}

pub trait BorrowMutAuthenticatorContext: BorrowMut<dyn AuthenticatorContext> {}
impl<T> BorrowMutAuthenticatorContext for T where T: BorrowMut<dyn AuthenticatorContext> {}
