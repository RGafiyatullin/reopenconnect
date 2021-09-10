use super::*;

use ::tokio::net::TcpStream;
use ::tokio_rustls::TlsConnector;
use ::webpki::DNSNameRef;

use crate::util::HttpIo;

pub type IO = Box<dyn HttpIo>;

pub async fn run(connect_to: &ConnectTo, tls_connector: &TlsConnector) -> Result<IO, AnyError> {
    let connect_to_domain = connect_to.http_host()?;
    let connect_to_host = connect_to.connect_to_host()?;
    let connect_to_port = connect_to.connect_to_port();

    let tcp_stream = TcpStream::connect((connect_to_host, connect_to_port)).await?;

    let maybe_tls_stream: Box<dyn HttpIo> = if connect_to.debug_no_tls {
        Box::new(tcp_stream)
    } else {
        let dns_name_ref = DNSNameRef::try_from_ascii_str(connect_to_domain)?;
        let tls_stream = tls_connector.connect(dns_name_ref, tcp_stream).await?;
        Box::new(tls_stream)
    };

    Ok(maybe_tls_stream)
}
