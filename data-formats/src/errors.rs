use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("Cryptographic error stack: {0}")]
    CryptoStack(#[from] openssl::error::ErrorStack),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_cbor::Error),
    #[error("COSE error: {0}")]
    Cose(#[from] aws_nitro_enclaves_cose::error::COSEError),
    #[error("Invalid hash value")]
    IncorrectHash,
    #[error("Unsupported algorithm used")]
    UnsupportedAlgorithm,
    #[error("Non-owner key attempted to sign")]
    NonOwnerKey,
    #[error("A previouw entry in the ownership voucher failed to validate")]
    PreviousEntryFailed,
    #[error("Inconsistent values were used")]
    InconsistentValue,
    #[error("An invalid state machine transition was attempted")]
    InvalidTransition,
}
