use std::error::Error as StdError;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::time::Duration;

use super::*;

use ::hyper::header::AsHeaderName;
use ::hyper::header::HeaderValue;
use ::ipnetwork::Ipv4Network;

use crate::protocol::cac::connection::CstpContext;
use crate::protocol::cac::ContextInvocationError;
use crate::util::HttpHeaders;
use crate::util::HttpIo;
use crate::AnyError;

use consts::cstp_headers;

impl<Ctx> Cstp<Ctx> {
    pub(super) async fn on_connected<IO>(
        io: IO,
        _token: &str,
        context: Arc<Mutex<Ctx>>,
        http_headers: HttpHeaders,
    ) -> Result<(Self, IO), CstpConnectError>
    where
        IO: HttpIo,
        Ctx: CstpContext,
    {
        let address_v4: Ipv4Addr = require_header(&http_headers, cstp_headers::CSTP_ADDR_V4)?;
        let netmask_v4: Ipv4Addr = require_header(&http_headers, cstp_headers::CSTP_MASK_V4)?;
        let dns_addrs: Vec<IpAddr> = require_headers(&http_headers, cstp_headers::CSTP_DNS_ADDRS)?;

        let idle_timeout = Duration::from_secs(require_header(
            &http_headers,
            cstp_headers::CSTP_IDLE_TIMEOUT,
        )?);
        let disconnected_timeout = Duration::from_secs(require_header(
            &http_headers,
            cstp_headers::CSTP_DISCONNECTED_TIMEOUT,
        )?);
        let keepalive =
            Duration::from_secs(require_header(&http_headers, cstp_headers::KEEPALIVE)?);

        let split_include: Vec<Ipv4Network> =
            require_headers(&http_headers, cstp_headers::SPLIT_INCLUDE)?;

        let cstp_props = CstpProps {
            address_v4,
            netmask_v4,
            dns_addrs,
            idle_timeout,
            disconnected_timeout,
            keepalive,
            split_include,
            http_headers,
        };

        {
            let mut ctx = context.lock().await;
            let () = ctx
                .cstp_connected(&cstp_props)
                .await
                .map_err(ContextInvocationError)?;
        }

        let cstp = Self {
            cstp_props,
            context,
        };

        Ok((cstp, io))
    }
}

fn require_header<K, V, E>(headers: &HttpHeaders, key: K) -> Result<V, CstpConnectError>
where
    K: AsHeaderName + Copy,
    V: FromStr<Err = E>,
    E: StdError + Send + Sync + 'static,
{
    let value = headers
        .get(key)
        .ok_or(CstpConnectError::HttpHeaderMissing)
        .and_then(parse_header_value)?;

    Ok(value)
}

fn require_headers<K, V, E>(headers: &HttpHeaders, key: K) -> Result<Vec<V>, CstpConnectError>
where
    K: AsHeaderName + Copy,
    V: FromStr<Err = E>,
    E: StdError + Send + Sync + 'static,
{
    let values = headers
        .get_all(key)
        .into_iter()
        .map(parse_header_value)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(values)
}

fn parse_header_value<V, E>(value: &HeaderValue) -> Result<V, CstpConnectError>
where
    V: FromStr<Err = E>,
    E: StdError + Send + Sync + 'static,
{
    let value =
        std::str::from_utf8(value.as_bytes()).map_err(CstpConnectError::HttpHeaderIsNotUtf8)?;
    let value = value
        .parse()
        .map_err(AnyError::from)
        .map_err(CstpConnectError::HeaderParseError)?;
    Ok(value)
}
