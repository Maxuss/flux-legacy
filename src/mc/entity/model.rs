use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use crate::mc::entity::{IntoSelector, Selector};
use crate::mc::world::WorldAccess;
use crate::modules::functions::FunctionWriter;
use crate::nbt;
use crate::nbt::{Compound, IntoTag, NbtTag};
use crate::prelude::EntityMeta;
