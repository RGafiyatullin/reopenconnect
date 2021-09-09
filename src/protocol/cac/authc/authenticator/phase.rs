#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuthenticationPhase {
    Init,
    Response,
    Connect,
}
