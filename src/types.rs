pub type Result<T> = anyhow::Result<T>;
pub type ChildPids = Vec<i32>;

#[derive(Debug, PartialEq)]
pub enum ListenStatus {
    Found,
    NotFound
}
