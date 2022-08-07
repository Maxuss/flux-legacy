use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

pub use lobsterchat::*;
use lobsterchat::component::Component;
use crate::nbt::NbtTag;

impl Into<NbtTag> for Component {
    fn into(self) -> NbtTag {
        NbtTag::String(self.to_string())
    }
}