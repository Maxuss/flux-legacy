use crate::nbt::{NbtTag, NbtWriter};
use std::fmt::Display;
use std::io::Write;

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
        for ele in &v {
            str.push_str(format!("{}", ele).as_str());

            if let Some(last) = v.last() {
                if *last != *ele {
                    str.push(',')
                }
            }
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
            NbtTag::String(v) => self.write(format!("\"{}\"", v))?,
            NbtTag::List(v) => {
                self.write_str("[")?;
                for ele in &v {
                    self.write_tag(None, ele.to_owned())?;
                    if let Some(last) = v.last() {
                        if *last != *ele {
                            self.write_str(",")?;
                        }
                    }
                }
                self.write_str("]")?;
            }
            NbtTag::Compound(comp) => {
                self.write_str("{")?;
                let iter = comp.iter();
                for (k, v) in iter {
                    self.write_tag(Some(k.to_owned()), v.to_owned())?;
                    if let Some((last_k, last_v)) = comp.iter().last() {
                        if last_k != k && last_v != v {
                            self.write_str(",")?;
                        }
                    }
                }
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
        };

        Ok(())
    }
}
