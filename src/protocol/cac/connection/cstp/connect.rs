use super::*;

use ::futures::prelude::*;

use ::hyper::client::conn as http_conn;
use ::hyper::header::HeaderValue;

use ::hyper::client::conn::Parts as HttpConnParts;
use ::hyper::client::conn::SendRequest as HttpConnApi;
use ::hyper::Body as HttpBody;
use ::hyper::Method as HttpMethod;
use ::hyper::Request as HttpRequest;
use ::hyper::StatusCode as HttpStatus;

use crate::protocol::cac::connection::CstpContext;
use crate::protocol::cac::ContextInvocationError;
use crate::protocol::cac::HttpError;
use crate::util::HttpHeaders;
use crate::util::HttpIo;

impl<Ctx> Cstp<Ctx> {
    pub async fn connect<IO>(
        io: IO,
        token: &str,
        context: Arc<Mutex<Ctx>>,
    ) -> Result<(Self, IO), CstpConnectError>
    where
        IO: HttpIo,
        Ctx: CstpContext,
    {
        log::info!("Connecting CSTP-channel [token: {:?}]", token);

        let mut ctx = context.lock().await;

        let (http_conn_api, http_conn) = http_conn::Builder::new()
            .http1_read_buf_exact_size(Some(consts::HTTP_READ_BUF_SIZE))
            // .http1_preserve_header_case(true)
            .http1_title_case_headers(true)
            .handshake::<_, HttpBody>(io)
            .await
            .map_err(HttpError::Handshake)?;

        let http_conn_running = http_conn.without_shutdown().map_err(HttpError::Connection);

        let mut http_request = HttpRequest::builder()
            .method(HttpMethod::CONNECT)
            .uri(consts::CSTP_PATH);

        let headers = {
            use consts::cstp_headers as consts;

            let base_mtu = ctx.base_mtu().await.map_err(ContextInvocationError)?;
            let mtu = ctx.mtu().await.map_err(ContextInvocationError)?;

            let mut headers = HttpHeaders::new();
            let _ = headers.append(
                "Host",
                HeaderValue::from_str(
                    &ctx.http_host()
                        .await
                        .map_err(ContextInvocationError)?
                        .as_str(),
                )
                .map_err(HttpError::HeaderValue)?,
            );
            let _ = headers.append(
                "Cookie",
                HeaderValue::from_str(&format!("webvpn={}", token))
                    .map_err(HttpError::HeaderValue)?,
            );
            let _ = headers.append(
                consts::CSTP_VERSION,
                HeaderValue::from_str("1").map_err(HttpError::HeaderValue)?,
            );

            let _ = headers.append(
                consts::CSTP_BASE_MTU,
                HeaderValue::from_str(&base_mtu.to_string()).map_err(HttpError::HeaderValue)?,
            );
            let _ = headers.append(
                consts::CSTP_MTU,
                HeaderValue::from_str(&mtu.to_string()).map_err(HttpError::HeaderValue)?,
            );
            let _ = headers.append(
                consts::CSTP_ADDRESS_TYPE,
                HeaderValue::from_str("IPv4").map_err(HttpError::HeaderValue)?,
            );

            let () = ctx
                .patch_http_headers(CstpPhase, &mut headers)
                .await
                .map_err(ContextInvocationError)?;

            headers
        };

        if let Some(hs) = http_request.headers_mut() {
            *hs = headers;
        }
        let http_request = http_request
            .body(HttpBody::empty())
            .map_err(HttpError::Request)?;

        let response_fut = process_request(http_conn_api, http_request);
        let http_conn_running = run_http_conn(http_conn_running);

        let ((http_status, http_headers, http_body), io) =
            future::try_join(response_fut, http_conn_running).await?;

        if http_status.is_success() {
            log::trace!("HTTP-Status: {:?}", http_status);
            log::trace!("HTTP-headers: {:#?}", http_headers);
            log::trace!("HTTP-body: ...");
            for line in http_body_readable(&http_body) {
                log::trace!("{}", line);
            }

            let () = std::mem::drop(ctx);
            let ret = Self::on_connected(io, token, context, http_headers).await?;

            Ok(ret)
        } else {
            log::error!("HTTP-Status: {:?}", http_status);
            log::error!("HTTP-headers: {:#?}", http_headers);
            log::error!("HTTP-body: {:?}", http_body_readable(&http_body));
            Err(CstpConnectError::HttpNon2xx(http_status))
        }
    }
}

async fn process_request(
    mut http_conn_api: HttpConnApi<HttpBody>,
    http_request: HttpRequest<HttpBody>,
) -> Result<(HttpStatus, HttpHeaders, Vec<u8>), HttpError> {
    let http_response = http_conn_api
        .send_request(http_request)
        .await
        .map_err(HttpError::ProcessRequest)?;
    let http_status = http_response.status().to_owned();
    let http_headers = http_response.headers().to_owned();
    let http_body = http_response.into_body();
    let http_body = ::hyper::body::to_bytes(http_body)
        .await
        .map_err(HttpError::Body)?;
    let http_body = http_body.to_vec();

    Ok((http_status, http_headers, http_body))
}

async fn run_http_conn<F, IO>(http_conn_running: F) -> Result<IO, HttpError>
where
    F: Future<Output = Result<HttpConnParts<IO>, HttpError>>,
{
    let conn_parts = http_conn_running.await?;
    log::trace!(
        "HTTP-CONNECT. Read-Buf is not empty: {:#02x?}",
        conn_parts.read_buf
    );
    // assert!(conn_parts.read_buf.is_empty());
    Ok(conn_parts.io)
}

fn http_body_readable(body: &[u8]) -> Vec<String> {
    if let Ok(body) = std::str::from_utf8(body) {
        body.to_owned().lines().map(ToOwned::to_owned).collect()
    } else {
        vec![format!("{:02x?}", body)]
    }
}
