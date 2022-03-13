pub mod nbt;
pub mod snbt;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::nbt::{BinaryNbtWriter, Compound, NbtTag, NbtWriter};
    use crate::utils::force_create;
    use std::collections::HashMap;
    use std::env::current_dir;
    use crate::snbt::StringNbtWriter;

    #[test]
    fn test_snbt() {
        let map = HashMap::<String, NbtTag>::from([
            ("byte".to_string(), NbtTag::Byte(120)),
            ("str".to_string(), NbtTag::String("A string!".to_string())),
            ("list".to_string(), NbtTag::List(vec![
                NbtTag::Int(0xFFAA),
                NbtTag::Int(0xAAFF),
                NbtTag::Int(0xAAAA)
            ])),
            ("comp".to_string(), NbtTag::Compound(Compound::new(HashMap::from([
                ("key".to_string(), NbtTag::String("value".to_string()))
            ]))))
        ]);
        let comp = Compound::new(map);
        let mut path = current_dir().unwrap();
        path.push("target/bin.nbt");
        let mut spath = current_dir().unwrap();
        spath.push("target/nbt.txt");
        let f = force_create(path);
        let sf = force_create(spath);
        let mut bw = BinaryNbtWriter::new(f);
        let mut sw = StringNbtWriter::new(sf);
        bw.write_tag(None, NbtTag::Compound(comp.to_owned())).unwrap();
        sw.write_tag(None, NbtTag::Compound(comp.to_owned())).unwrap();
    }
}
