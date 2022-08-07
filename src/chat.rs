use crate::nbt::NbtTag;
use lobsterchat::component::Component;
pub use lobsterchat::*;

impl Into<NbtTag> for Component {
    fn into(self) -> NbtTag {
        NbtTag::String(self.to_string())
    }
}
