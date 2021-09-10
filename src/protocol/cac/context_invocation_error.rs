use crate::AnyError;

#[derive(Debug, ::thiserror::Error)]
#[error("Context Invocation Error")]
pub struct ContextInvocationError(#[source] pub AnyError);
