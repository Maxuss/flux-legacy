use crate::nbt::NbtTag;
use std::fmt::{Display, Formatter};

pub mod block;
pub mod commands;
pub mod context;
pub mod data;
pub mod enchant;
pub mod entity;
pub mod item;
pub mod material;
pub mod world;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Identifier {
    namespace: String,
    path: String,
}

impl Identifier {
    pub fn new<S>(namespace: S, path: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            namespace: namespace.into(),
            path: path.into(),
        }
    }

    pub fn minecraft<S>(path: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            namespace: "minecraft".to_string(),
            path: path.into(),
        }
    }
}

impl Into<String> for Identifier {
    fn into(self) -> String {
        self.namespace + ":" + self.path.as_str()
    }
}

impl Into<NbtTag> for Identifier {
    fn into(self) -> NbtTag {
        NbtTag::String(self.to_string())
    }
}

pub trait Identified {
    fn id(&self) -> Identifier;
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}:{}", self.namespace, self.path))
    }
}
