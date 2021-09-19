use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("No child process found")]
    NoChildProcesses,

    #[error("Cannot execute empty command")]
    EmptyCommand
}
