pub enum RustyNailError {
    CommunicationFailed,
    GenericError,
}

pub type RustyNailResult<T> = Result<T, RustyNailError>;
