use std::fmt::Display;
use std::io::Write;
use crate::nbt::{NbtTag, NbtWriter};
use crate::utils::escape;

pub struct StringNbtWriter<W> {
    write: W,
}

impl<W> StringNbtWriter<W>
where
    W: Write,
{
    pub fn new(write: W) -> Self {
        Self { write }
    }

    fn write_str(&mut self, str: &str) -> anyhow::Result<()> {
        self.write
            .write(str.as_bytes())
            .map_err(anyhow::Error::from)
            .map(|_| ())
    }

    fn write(&mut self, str: String) -> anyhow::Result<()> {
        self.write
            .write(str.as_bytes())
            .map_err(anyhow::Error::from)
            .map(|_| ())
    }

    fn write_vec<T>(&mut self, str: &mut String, v: Vec<T>) -> anyhow::Result<()>
    where
        T: Display + PartialEq,
    {
        let mut iter = v.iter().peekable();
        while let Some(ele) = iter.next() {
            str.push_str(format!("{}", ele).as_str());

            if iter.peek().is_some() {
                str.push_str(",");
            };
        }
        str.push_str("]");
        self.write(str.to_owned())
    }
}

impl<W> NbtWriter for StringNbtWriter<W>
where
    W: Write,
{
    fn write_tag(&mut self, name: Option<String>, tag: NbtTag) -> anyhow::Result<()> {
        if tag == NbtTag::Empty {
            return Ok(())
        };
        if let Some(name) = name {
            self.write(format!("{}:", name))?;
        };

        match tag {
            NbtTag::Byte(v) => self.write(format!("{}b", v))?,
            NbtTag::Short(v) => self.write(format!("{}s", v))?,
            NbtTag::Int(v) => self.write(format!("{}", v))?,
            NbtTag::Long(v) => self.write(format!("{}L", v))?,
            NbtTag::Float(v) => self.write(format!("{}f", v))?,
            NbtTag::Double(v) => self.write(format!("{}d", v))?,
            NbtTag::ByteArray(v) => {
                let mut str = String::from("[");
                self.write_vec(&mut str, v)?;
            }
            NbtTag::String(v) => self.write(format!("\"{}\"", escape(v)))?,
            NbtTag::List(v) => {
                self.write_str("[")?;
                let mut iter = v.iter().peekable();
                while let Some(ele) = iter.next() {
                    self.write_tag(None, ele.to_owned())?;
                    if iter.peek().is_some() {
                        self.write_str(",");
                    };
                }
                self.write_str("]")?;
            }
            NbtTag::Compound(comp) => {
                self.write_str("{")?;
                let mut iter = comp.iter().peekable();
                while let Some((k, v)) = iter.next() {
                    if v.to_owned() == NbtTag::Empty {
                        continue
                    }
                    self.write_tag(Some(k.to_owned()), v.to_owned())?;
                    if iter.peek().is_some() {
                        self.write_str(",");
                    }
                };
                self.write_str("}")?;
            }
            NbtTag::IntArray(v) => {
                let mut str = String::from("[I;");
                self.write_vec(&mut str, v)?;
            }
            NbtTag::LongArray(v) => {
                let mut str = String::from("[L;");
                self.write_vec(&mut str, v)?;
            }
            _ => {}
        };

        Ok(())
    }
}
