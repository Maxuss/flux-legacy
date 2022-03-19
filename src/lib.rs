pub mod macros;
pub mod modules;
pub mod nbt;
pub mod snbt;
pub mod mc;
pub mod prelude;
pub mod chat;
pub mod utils;
#[cfg(test)]
mod tests {
    use crate::nbt;
    use crate::nbt::{BinaryNbtWriter, Compound, IntoTag, NbtTag, NbtWriter};
    use crate::snbt::StringNbtWriter;
    use crate::utils::force_create;
    use std::collections::HashMap;
    use std::env::current_dir;

    #[tokio::test]
    async fn test_nbt() {
        let map = HashMap::<String, NbtTag>::from([
            ("byte".to_string(), NbtTag::Byte(120)),
            ("str".to_string(), NbtTag::String("A string!".to_string())),
            (
                "list".to_string(),
                NbtTag::List(vec![
                    NbtTag::Int(0xFFAA),
                    NbtTag::Int(0xAAFF),
                    NbtTag::Int(0xAAAA),
                ]),
            ),
            (
                "comp".to_string(),
                NbtTag::Compound(Compound::new(HashMap::from([(
                    "key".to_string(),
                    NbtTag::String("value".to_string()),
                )]))),
            ),
        ]);
        let comp = Compound::new(map);
        let mut path = current_dir().unwrap();
        path.push("target/bin.nbt");
        let mut spath = current_dir().unwrap();
        spath.push("target/nbt.txt");
        let f = force_create(path).await;
        let sf = force_create(spath).await;
        let mut bw = BinaryNbtWriter::new(f.into_std().await);
        let mut sw = StringNbtWriter::new(sf.into_std().await);
        bw.write_tag(None, NbtTag::Compound(comp.to_owned()))
            .unwrap();
        sw.write_tag(None, NbtTag::Compound(comp.to_owned()))
            .unwrap();
    }

    #[tokio::test]
    async fn test_macros() {
        let key = "Proc-Key";
        let comp = nbt! {
            key: "value", // Normal key + value
            "Another-Key": "value", // Ket that is already a string
            double: 12.12_f64, // Doubles / floats / ints
            float: 12.12_f32,
            int: 12000,
            comp: {
                key: "value"
            }, // inner compounds
            list: [120, 130, 0xFAF], // using normal lists
            int_array: [I; 120, 140], // int arrays (as specified by I; infix)
            long_array: [L; 5000000, 12000000], // long arrays (L; infix)
            byte_array: [B; 0x5A, 0x1B], // byte arrays (B; infix)
            [key]: 1200i64 // using key as variable by containing it in brackets
        }
            .nbt();
        let mut path = current_dir().unwrap();
        path.push("target/bin.nbt");
        let f = force_create(path).await;
        let mut writer = BinaryNbtWriter::new(f.into_std().await);
        writer.write_tag(None, comp).unwrap();
    }
}
