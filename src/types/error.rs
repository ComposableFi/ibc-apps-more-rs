// use displaydoc::Display;

#[derive(Debug)]
// #[derive(Display)]
pub enum HookError {
    /// Error encoding bech32: {0}
    Bech32EncodeError(bech32::EncodeError),
    /// Error making prefix: {0}
    Bech32ParseError(bech32::primitives::hrp::Error),
}

impl From<bech32::EncodeError> for HookError {
    fn from(e: bech32::EncodeError) -> Self {
        HookError::Bech32EncodeError(e)
    }
}

impl From<bech32::primitives::hrp::Error> for HookError {
    fn from(e: bech32::primitives::hrp::Error) -> Self {
        HookError::Bech32ParseError(e)
    }
}
