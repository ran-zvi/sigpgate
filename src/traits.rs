use crate::types::{ListenStatus, Result};

pub trait Listen {
    fn listen(&self) -> Result<ListenStatus>;
}