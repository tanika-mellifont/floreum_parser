use crate::Response;
#[derive(Clone, PartialEq, Eq)]
pub struct ResponseError {}
impl ResponseError {
    pub const KIND_TAG: u64 = 1;
    pub fn new() -> Self {
        Self {}
    }
}
