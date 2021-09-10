use super::*;

use crate::protocol::cac::authc::*;
use crate::util::HttpHeaders;

#[async_trait::async_trait]
impl AuthenticatorContext for Context {
    async fn patch_version(
        &mut self,
        _phase: AuthenticationPhase,
        version: &mut client_pdu::common::Version,
    ) -> Result<(), AnyError> {
        if let Some(authc_version) = self.args.authc_version() {
            version.ver = authc_version.ver.to_owned();
        }
        Ok(())
    }
    async fn patch_device_id(
        &mut self,
        _phase: AuthenticationPhase,
        device_id: &mut client_pdu::common::DeviceID,
    ) -> Result<(), AnyError> {
        if let Some(authc_device_id) = self.args.authc_device_id() {
            for kv in &authc_device_id.add_device_id_attr {
                let _ = device_id
                    .extra_attrs
                    .insert(kv.key.to_owned(), kv.value.to_owned());
            }
            if let Some(platform) = authc_device_id.device_id_platform.as_ref() {
                device_id.platform = platform.to_owned();
            }
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl AuthenticatorContextInit for Context {
    async fn patch_init_rq(
        &mut self,
        init_rq: &mut client_pdu::AuthConfigInit,
    ) -> Result<(), AnyError> {
        if let Some(authc_rq) = self.args.authc_rq() {
            for kv in &authc_rq.add_rq_attr {
                let _ = init_rq
                    .extra_attrs
                    .insert(kv.key.to_owned(), kv.value.to_owned());
            }
            for ch in &authc_rq.add_rq_child {
                let () = init_rq.extra_children.push(Box::new(ch.to_owned()));
            }
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl AuthenticatorContextRequest for Context {
    async fn received_request(
        &mut self,
        _http_headers: &HttpHeaders,
        req: &server_pdu::AuthConfigRequest,
    ) -> Result<(), AnyError> {
        let () = req
            .extra_children
            .iter()
            .filter(|e| e.name() == consts::config_auth_opaque::OPAQUE)
            .filter_map(|opaque| {
                let is_for = opaque.attr(consts::config_auth_opaque::IS_FOR);
                is_for.map(|is_for| (is_for, opaque.to_owned()))
            })
            .for_each(|(is_for, opaque)| {
                let _ = self.opaques.insert(is_for.to_owned(), opaque);
            });
        Ok(())
    }
}

#[async_trait::async_trait]
impl AuthenticatorContextReply for Context {
    async fn patch_reply_rq(
        &mut self,
        reply_rq: &mut client_pdu::AuthConfigReply,
    ) -> Result<(), AnyError> {
        if let Some(authc_rq) = self.args.authc_rq() {
            for kv in &authc_rq.add_rq_attr {
                let _ = reply_rq
                    .extra_attrs
                    .insert(kv.key.to_owned(), kv.value.to_owned());
            }
            for ch in &authc_rq.add_rq_child {
                let () = reply_rq.extra_children.push(Box::new(ch.to_owned()));
            }
        }
        if let Some(authc_reply) = self.args.authc_reply() {
            for auth_reply_node in &authc_reply.nodes {
                reply_rq.auth_reply_nodes.push(auth_reply_node.to_owned());
            }
        }

        for (_, opaque) in &self.opaques {
            let () = reply_rq.extra_children.push(Box::new(opaque.to_owned()));
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl AuthenticatorContextComplete for Context {
    async fn received_complete(
        &mut self,
        _http_headers: &HttpHeaders,
        _req: &server_pdu::AuthConfigComplete,
    ) -> Result<(), AnyError> {
        Ok(())
    }
}
