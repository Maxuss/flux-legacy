use anyhow::bail;
use byteorder::{BigEndian, WriteBytesExt};
use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use std::io::Write;

macro_rules! bare_fn {
    ($(
    $typ:ty = {
        $write:ident as $wname:ident
    });* $(;)*) =>
    {
        $(
        fn $wname(bare: $typ, write: &mut impl Write) -> anyhow::Result<()> {
            write.$write::<BigEndian>(bare).map_err(anyhow::Error::from)
        }
        )*
    };
}

fn write_byte(bare: i8, write: &mut impl Write) -> anyhow::Result<()> {
    write.write_i8(bare).map_err(anyhow::Error::from)
}

bare_fn! {
    i16 = {
        write_i16 as write_short
    };
    i32 = {
        write_i32 as write_int
    };
    i64 = {
        write_i64 as write_long
    };
    f32 = {
        write_f32 as write_float
    };
    f64 = {
        write_f64 as write_double
    };
}

fn write_byte_array(arr: &[i8], write: &mut impl Write) -> anyhow::Result<()> {
    write.write_i32::<BigEndian>(arr.len() as i32)?;
    for &v in arr {
        write_byte(v, write)?;
    }
    Ok(())
}

fn write_int_array(arr: &[i32], write: &mut impl Write) -> anyhow::Result<()> {
    write.write_i32::<BigEndian>(arr.len() as i32)?;
    for &v in arr {
        write_int(v, write)?;
    }
    Ok(())
}

fn write_long_array(arr: &[i64], write: &mut impl Write) -> anyhow::Result<()> {
    write.write_i32::<BigEndian>(arr.len() as i32)?;
    for &v in arr {
        write_long(v, write)?;
    }
    Ok(())
}

fn write_string(str: String, write: &mut impl Write) -> anyhow::Result<()> {
    write_short(str.len() as i16, write)?;
    write.write_all(str.as_bytes()).map_err(anyhow::Error::from)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Compound {
    tags: HashMap<String, NbtTag>,
}

impl Compound {
    pub fn new(tags: HashMap<String, NbtTag>) -> Self {
        Self { tags }
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, String, NbtTag> {
        self.tags.iter()
    }
}

impl From<HashMap<String, NbtTag>> for Compound {
    fn from(map: HashMap<String, NbtTag>) -> Self {
        Compound::new(map)
    }
}

impl Into<HashMap<String, NbtTag>> for Compound {
    fn into(self) -> HashMap<String, NbtTag> {
        self.tags
    }
}

impl IntoIterator for Compound {
    type Item = (String, NbtTag);
    type IntoIter = IntoIter<String, NbtTag>;

    fn into_iter(self) -> Self::IntoIter {
        self.tags.into_iter()
    }
}

/// Trait used to convert structs into traits
pub trait IntoTag {
    /// Converts this element to Nbt Tag
    fn nbt(self) -> NbtTag;
}

impl<T> IntoTag for T where T: Into<NbtTag> {
    fn nbt(self) -> NbtTag {
        self.into()
    }
}

impl IntoTag for Compound {
    fn nbt(self) -> NbtTag {
        NbtTag::Compound(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NbtTag {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<NbtTag>),
    Compound(Compound),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl<S, V> Into<NbtTag> for HashMap<S, V> where S: Into<String>, V: Into<NbtTag> {
    fn into(self) -> NbtTag {
        let map = self.into_iter().map(|(k, v)| (k.into(), v.into())).collect::<HashMap<String, NbtTag>>();
        NbtTag::Compound(Compound::new(map))
    }
}

impl Into<NbtTag> for i8 {
    fn into(self) -> NbtTag {
        NbtTag::Byte(self)
    }
}

impl Into<NbtTag> for i16 {
    fn into(self) -> NbtTag {
        NbtTag::Short(self)
    }
}

impl Into<NbtTag> for i32 {
    fn into(self) -> NbtTag {
        NbtTag::Int(self)
    }
}

impl Into<NbtTag> for i64 {
    fn into(self) -> NbtTag {
        NbtTag::Long(self)
    }
}

impl Into<NbtTag> for f32 {
    fn into(self) -> NbtTag {
        NbtTag::Float(self)
    }
}

impl Into<NbtTag> for f64 {
    fn into(self) -> NbtTag {
        NbtTag::Double(self)
    }
}

impl Into<NbtTag> for String {
    fn into(self) -> NbtTag {
        NbtTag::String(self)
    }
}

impl Into<NbtTag> for &str {
    fn into(self) -> NbtTag {
        NbtTag::String(self.to_string())
    }
}

impl<N> Into<NbtTag> for Vec<N> where N: Into<NbtTag> {
    fn into(self) -> NbtTag {
        NbtTag::List(self.into_iter().map(|e| e.into()).collect::<Vec<NbtTag>>())
    }
}

impl NbtTag {
    pub fn id(&self) -> u8 {
        match *self {
            NbtTag::Byte(_) => 0x01,
            NbtTag::Short(_) => 0x02,
            NbtTag::Int(_) => 0x03,
            NbtTag::Long(_) => 0x04,
            NbtTag::Float(_) => 0x05,
            NbtTag::Double(_) => 0x06,
            NbtTag::ByteArray(_) => 0x07,
            NbtTag::String(_) => 0x08,
            NbtTag::List(_) => 0x09,
            NbtTag::Compound(_) => 0x0a,
            NbtTag::IntArray(_) => 0x0b,
            NbtTag::LongArray(_) => 0x0c,
        }
    }
}

pub trait NbtWriter {
    fn write_tag(&mut self, name: Option<String>, tag: NbtTag) -> anyhow::Result<()>;
}

pub struct BinaryNbtWriter<W> {
    write: W,
}

impl<W> BinaryNbtWriter<W>
where
    W: Write,
{
    pub fn new(write: W) -> Self {
        Self { write }
    }

    fn _nn_write_tag(&mut self, tag: NbtTag) -> anyhow::Result<()> {
        match tag {
            NbtTag::Byte(v) => write_byte(v, &mut self.write)?,
            NbtTag::Short(v) => write_short(v, &mut self.write)?,
            NbtTag::Int(v) => write_int(v, &mut self.write)?,
            NbtTag::Long(v) => write_long(v, &mut self.write)?,
            NbtTag::Float(v) => write_float(v, &mut self.write)?,
            NbtTag::Double(v) => write_double(v, &mut self.write)?,
            NbtTag::ByteArray(v) => write_byte_array(&v, &mut self.write)?,
            NbtTag::String(v) => write_string(v, &mut self.write)?,
            NbtTag::List(v) => {
                if v.is_empty() {
                    self.write.write_u8(0u8)?;
                    self.write.write_i32::<BigEndian>(0)?;
                } else {
                    let ty = v[0].id();
                    self.write.write_u8(ty)?;
                    self.write.write_i32::<BigEndian>(v.len() as i32)?;
                    for ele in v {
                        if ele.id() != ty {
                            bail!("List with more than single type provided!")
                        }

                        self._nn_write_tag(ele)?;
                    }
                }
            }
            NbtTag::Compound(v) => {
                for (k, v) in v.iter() {
                    self.write_tag(Some(k.to_owned()), v.to_owned())?;
                }
                self.write.write_u8(0x00)?;
            }
            NbtTag::IntArray(v) => write_int_array(&v, &mut self.write)?,
            NbtTag::LongArray(v) => write_long_array(&v, &mut self.write)?,
        };
        Ok(())
    }
}

impl<W> NbtWriter for BinaryNbtWriter<W>
where
    W: Write,
{
    fn write_tag(&mut self, name: Option<String>, tag: NbtTag) -> anyhow::Result<()> {
        self.write.write_u8(tag.id())?;
        if let Some(name) = name {
            write_string(name, &mut self.write)?;
        } else {
            write_string(String::new(), &mut self.write)?;
        }
        self._nn_write_tag(tag)
    }
}
