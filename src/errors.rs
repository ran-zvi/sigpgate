use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("Cannot execute empty command")]
    EmptyCommand
}
