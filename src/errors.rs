use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("No child process found")]
    NoChildProcesses
}