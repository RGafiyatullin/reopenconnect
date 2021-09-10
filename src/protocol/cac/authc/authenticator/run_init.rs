use std::convert::TryFrom;

use ::hyper::header::HeaderValue;

use crate::protocol::cac::authc::client_pdu;
use crate::protocol::cac::authc::server_pdu::AuthConfigServerPDU;
use crate::util::HttpHeaders;

use super::*;

impl<IO, Ctx> Authenticator<IO, Ctx>
where
    IO: HttpIo,
    Ctx: BorrowMutAuthenticatorContext,
{
    pub(super) async fn run_init(
        &mut self,
    ) -> Result<(HttpHeaders, AuthConfigServerPDU), AuthenticatorError> {
        let context = self.context.borrow_mut();

        let http_host = context.http_host().await.map_err(ContextInvocationError)?;

        let method = HttpMethod::POST;
        let path = "/";

        let mut headers = HttpHeaders::new();

        headers.append(
            "Host",
            HeaderValue::from_str(&http_host).map_err(HttpError::HeaderValue)?,
        );
        headers.append(
            "Connection",
            HeaderValue::from_str("Keep-Alive").map_err(HttpError::HeaderValue)?,
        );
        headers.append(
            "Accept",
            HeaderValue::from_str("*/*").map_err(HttpError::HeaderValue)?,
        );
        headers.append(
            "Accept-Encoding",
            HeaderValue::from_str("identity").map_err(HttpError::HeaderValue)?,
        );
        headers.append(
            "Content-Type",
            HeaderValue::from_str("application/x-www-form-urlencoded")
                .map_err(HttpError::HeaderValue)?,
        );

        let () = context
            .patch_http_headers(AuthenticationPhase::Init, &mut headers)
            .await
            .map_err(ContextInvocationError)?;

        let mut version = client_pdu::common::Version::new("0.1");
        let () = context
            .patch_version(AuthenticationPhase::Init, &mut version)
            .await
            .map_err(ContextInvocationError)?;

        let mut device_id = client_pdu::common::DeviceID::new("turing-machine");
        let () = context
            .patch_device_id(AuthenticationPhase::Init, &mut device_id)
            .await
            .map_err(ContextInvocationError)?;

        let mut group_access =
            client_pdu::common::GroupAccess::new(format!("https://{}", http_host));
        let () = context
            .group_access(AuthenticationPhase::Init, &mut group_access)
            .await
            .map_err(ContextInvocationError)?;

        let mut init_rq = client_pdu::AuthConfigInit {
            extra_attrs: Default::default(),
            version,
            device_id,
            group_access,
            extra_children: Default::default(),
        };
        let () = context
            .patch_init_rq(&mut init_rq)
            .await
            .map_err(ContextInvocationError)?;

        log::trace!("Rq\n{:?}\n{:#?}\n{:#?}", method, headers, init_rq);

        let http_request = self.create_request(method, path, headers, Some(init_rq))?;
        let http_response = self.process_request(http_request).await?;

        let http_status = http_response.status();
        let http_headers = http_response.headers().to_owned();

        log::trace!("response-status: {:?}", http_status);
        log::trace!("response-headers: {:#?}", http_headers);

        let body_xml = self
            .collect_body_xml(http_status, http_response.into_body())
            .await?;

        let server_pdu =
            AuthConfigServerPDU::try_from(body_xml).map_err(AuthenticatorError::CACProtocol)?;

        Ok((http_headers, server_pdu))
    }
}
