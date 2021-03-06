//! Contains the different error codes that were in psm.h in the C version.
pub enum Error {
    Ok,
    NoProgress,
    ParamError,
    NoMemory,
    NotIinitalized,
    BadApiVersion,
    NoAffinity,
    InternalError,
    ShmemSegmentError,
    OptReadOnly,
    Timeout,
    TooManyEps,
    Finalized,
    EpClosed,
    NoDevice,
    UnitNotFound,
    DeviceFailure,
    CloseTimeout,
    NoPortsAvailable,
    NoNetwork,
    InvalidUuidKey,
    EpNoResources,
    UnkownEpid,
    UnreachableEpid,
    InvalidNode,
    InvalidMtu,
    InvalidVersion,
    InvalidConnect,
    AlreadyConnected,
    NetworkError,
    InvalidPkey,
    EpidPathResolution,
    MqNoCompletions,
    MqTruncation,
    AmInvalidReply,
    AlreadyInitialized,
    UnknownError
}

pub fn error_to_string(error: Error) -> &'static str {
  "Unknown Error"
}
