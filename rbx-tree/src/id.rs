use serde_derive::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RbxId(Uuid);

impl RbxId {
    pub fn new() -> RbxId {
        RbxId(Uuid::new_v4())
    }

    pub fn parse_str(input: &str) -> Option<RbxId> {
        Uuid::parse_str(input).map(RbxId).ok()
    }
}