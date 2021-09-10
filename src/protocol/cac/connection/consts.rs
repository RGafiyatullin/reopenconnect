pub const CSTP_PATH: &str = "/CSCOSSLC/tunnel";
pub const HTTP_READ_BUF_SIZE: usize = 16 * 1024 * 1024;

pub mod cstp_headers {
    pub const CSTP_VERSION: &str = "X-CSTP-Version";
    pub const CSTP_ACCEPT_ENCODING: &str = "X-CSTP-Accept-Encoding";
    pub const CSTP_HOSTNAME: &str = "X-CSTP-Hostname";
    pub const CSTP_BASE_MTU: &str = "X-CSTP-Base-MTU";
    pub const CSTP_MTU: &str = "X-CSTP-MTU";
    pub const CSTP_ADDRESS_TYPE: &str = "X-CSTP-Address-Type";
    pub const CSTP_FULL_IPV6_CAPABILITY: &str = "X-CSTP-Full-IPv6-Capability";
    pub const CSTP_ADDR_V4: &str = "X-CSTP-Address";
    pub const CSTP_MASK_V4: &str = "X-CSTP-Netmask";
    pub const CSTP_DNS_ADDRS: &str = "X-CSTP-DNS";
    pub const CSTP_LEASE_DURATION: &str = "X-CSTP-Lease-Duration";
    pub const CSTP_SESSION_TIMEOUT: &str = "X-CSTP-Session-Timeout";
    pub const CSTP_SESSION_TIMEOUT_ALERT_INTERVAL: &str = "X-CSTP-Session-Timeout-Alert-Interval";
    pub const CSTP_SESSION_TIMEOUT_REMAINING: &str = "X-CSTP-Session-Timeout-Remaining";
    pub const CSTP_IDLE_TIMEOUT: &str = "X-CSTP-Idle-Timeout";
    pub const CSTP_DISCONNECTED_TIMEOUT: &str = "X-CSTP-Disconnected-Timeout";
    pub const CSTP_KEEP: &str = "X-CSTP-Keep";
    pub const CSTP_TUNNEL_ALL_DNS: &str = "X-CSTP-Tunnel-All-DNS";
    pub const CSTP_DPD: &str = "X-CSTP-DPD";
    pub const CSTP_TCP_KEEPALIVE: &str = "X-CSTP-TCP-Keepalive";
    pub const CSTP_LOCAL_ADDRESS_IP4: &str = "X-CSTP-Local-Address-IP4";
    pub const CSTP_REMOTE_ADDRESS_IP4: &str = "X-CSTP-Remote-Address-IP4";

    pub const DTLS_MASTER_SECRET: &str = "X-DTLS-Master-Secret";
    pub const DTLS_CIPHER_SUITE: &str = "X-DTLS-CipherSuite";
    pub const DTLS12_CIPHER_SUITE: &str = "X-DTLS12-CipherSuite";
    pub const DTLS_ACCEPT_ENCODING: &str = "X-DTLS-Accept-Encoding";
    pub const DTLS_SESSION_ID: &str = "X-DTLS-Session-ID";
    pub const DTLS_KEEPALIVE: &str = "X-DTLS-Keepalive";
    pub const DTLS_DPD: &str = "X-DTLS-DPD";
    pub const DTLS_MTU: &str = "X-DTLS-MTU";

    pub const KEEPALIVE: &str = "X-CSTP-Keepalive";
    pub const SPLIT_INCLUDE: &str = "X-CSTP-Split-Include";
}

pub mod cstp_frame {
    pub const MAGIC: [u8; 4] = [0x53, 0x54, 0x46, 0x01];

    pub const TYPE_DATA: u8 = 0x00;
    pub const TYPE_DPD_REQ: u8 = 0x03;
    pub const TYPE_DPD_RESP: u8 = 0x04;
    pub const TYPE_DISCONNECT: u8 = 0x05;
    pub const TYPE_KEEPALIVE: u8 = 0x07;
    pub const TYPE_COMPRESSED: u8 = 0x08;
    pub const TYPE_TERMINATE: u8 = 0x09;
}
