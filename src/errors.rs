use thiserror::Error;

#[derive(Error, Debug)]
pub enum SignalError {
    #[error("Signal id: {0} is invalid")]
    InvalidSignalId(u8)
}

#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("No child process found")]
    NoChildProcesses
}