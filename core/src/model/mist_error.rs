use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MistError {
    GeneralError(String),
    ChunklistValidationError(String),
    FileSizeAttributesError(String),
    InvalidData,
    InvalidDestinationUrl,
    InvalidDownloadResumeData,
    InvalidFileSize {
        invalid: u64,
        valid: u64,
    },
    InvalidShasum {
        invalid: String,
        valid: String,
    },
    InvalidTerminationStatus {
        status: i32,
        output: Option<String>,
        error: Option<String>,
    },
    InvalidUrl(String),
    MaximumRetriesReached,
    MissingDevicesKey,
    MissingFileAttributes,
    OutputStreamBufferError,
    OutputStreamWriteError,
    UserCancelled,
}
