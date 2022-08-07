use std::fmt::Pointer;
use std::io::Write;
use std::sync::{Arc, Mutex};
use crate::mc::entity::{Entity, IntoSelector, Selector};
use crate::mc::entity::meta::Allay;
use crate::mc::Identified;
use crate::modules::functions::FunctionWriter;
use crate::nbt::{IntoTag, NbtTag};
use crate::prelude::{CommandLike, EntityMeta, EntityType, Location};

#[derive(Debug, Clone)]
pub struct WorldAccess<W> {
    writer: Arc<Mutex<FunctionWriter<W>>>
}

impl<W> WorldAccess<W> where W: Write {
    pub fn new(writer: Arc<Mutex<FunctionWriter<W>>>) -> Self {
        Self { writer }
    }

    pub fn execute(&mut self, cmd: &mut impl CommandLike) {
        self.write_line(cmd.compile())
    }

    pub fn summon_entity(&mut self, at: Location, entity: Entity) {
        let ty = entity.get_type();
        let mut meta = entity.meta;
        let mut metastr = meta.compile();
        if metastr.contains("Tags:[") {
            metastr = metastr.replacen("Tags:[", &format!("Tags:[\"fluxd{}\"", entity.id), 1)
        } else {
            let comma = if metastr.len() > 2 { "," } else { "" };
            metastr = metastr.replacen("{", &format!("{{Tags:[\"fluxd{}\"]{}", entity.id, comma), 1)
        }
        self.write_line(format!("summon {} {} {}", ty.id(), at.to_string(), metastr))
    }

    pub fn write_line<S: Into<String>>(&mut self, line: S) {
        self.writer.lock().unwrap().write_line(format!("{}\n", line.into())).expect("Could not write line to world access!");
    }
}
