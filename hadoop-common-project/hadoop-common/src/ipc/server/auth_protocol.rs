#[derive(PartialEq, Eq)]
pub(crate) enum AuthProtocol {
    None,
    Sasl,
}

impl AuthProtocol {
    pub fn call_id(&self) -> i8 {
        match self {
            Self::None => 0,
            Self::Sasl => -33,
        }
    }
}
