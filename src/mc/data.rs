use crate::mc::Identifier;
use crate::modules::functions::FunctionWriter;
use crate::nbt::{IntoTag, NbtTag};
use anyhow::bail;
use std::collections::HashMap;
use std::io::Write;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct DataAccess<W> {
    name: Identifier,
    cache: HashMap<String, NbtTag>,
    writer: Arc<Mutex<FunctionWriter<W>>>,
}

impl<W> DataAccess<W>
where
    W: Write,
{
    pub fn new(named: Identifier, writer: Arc<Mutex<FunctionWriter<W>>>) -> Self {
        Self {
            name: named,
            cache: Default::default(),
            writer,
        }
    }

    pub fn set<K: Into<String>, V: IntoTag>(&mut self, key: K, value: V) -> anyhow::Result<()> {
        let k = key.into();
        let v = value.nbt();

        let _ = self.cache.insert(k.clone(), v.clone());
        self.writer
            .lock()
            .unwrap()
            .write_line(format!(
                "data modify storage {} {} set value {}",
                self.name,
                k,
                v.stringify()
            ))
            .map_err(|it| anyhow::Error::from(it))?;

        Ok(())
    }

    pub fn get<K: Into<String>>(&self, key: K) -> anyhow::Result<NbtTag> {
        let strkey = key.into();
        if let Some(v) = self.cache.get(&strkey) {
            Ok(v.clone())
        } else {
            bail!("Data storage did not contain tag with key {}", strkey)
        }
    }

    pub fn clear(&mut self) {
        self.cache.clear();
        self.writer.lock().unwrap().close()
    }
}
