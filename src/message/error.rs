#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResponseError {}
impl ResponseError {
    pub const KIND_TAG: u64 = 1;
    pub fn new() -> Self {
        Self {}
    }
}
