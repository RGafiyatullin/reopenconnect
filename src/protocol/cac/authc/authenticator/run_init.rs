use super::*;

impl<IO, Ctx> Authenticator<IO, Ctx>
where
    IO: HttpIo,
    Ctx: BorrowMutAuthenticatorContext,
{
    pub async fn run_init(&mut self) -> Result<(), AnyError> {
        let context = self.context.borrow_mut();
        let method = HttpMethod::POST;
        let path = "/";
        let headers = vec![
            (str!("Host"), context.http_host().await),
            (str!("User-Agent"), context.user_agent().await),
            (str!("Connection"), str!("Keep-Alive")),
            (str!("Accept"), str!("*/*")),
            (str!("Accept-Encoding"), str!("identity")),
            (
                str!("Content-Type"),
                str!("application/x-www-form-urlencoded"),
            ),
        ];
        let headers = context
            .patch_http_headers(AuthenticationPhase::Init, headers)
            .await;

        let version = context.version().await;
        let device_id = context.device_id().await;
        let group_access = context.group_access().await;

        let body = request::InitRequest {
            version,
            device_id,
            group_access,
            extra_children: Default::default(),
        };

        let http_request = self.create_request(method, path, headers, Some(body))?;
        let http_response = self.process_request(http_request).await?;

        unimplemented!()
    }
}
