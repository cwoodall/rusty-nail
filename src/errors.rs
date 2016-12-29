pub enum RustyNailError {
    CommunicationFailed,
    DispenserEmpty,
    NotEnoughLiquid,
    NotImplemented,
    NoPwmDriver,
    GenericError,
}

pub type RustyNailResult<T> = Result<T, RustyNailError>;
