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
    use std::collections::HashMap;
    use std::env::current_dir;
    use crate::chat::{Component, NamedColor};
    use crate::mc::enchant::{Enchant, Enchantment};
    use crate::mc::item::{DefaultMeta, ItemDisplay, ItemMeta};
    use crate::prelude::{ItemStack, Material};

    #[test]
    fn test_items() {
        let mut item = ItemStack::new(Material::DiamondSword, 1);
        let mut meta = DefaultMeta::new();
        meta.enchants(vec![Enchantment::new(Enchant::Sharpness, 5)]);
        meta.unbreakable(true);
        let mut display = ItemDisplay::new();
        display.name(Component::text("Amazing sword").color(NamedColor::Gold).bold(true));
        meta.display(display);
        item.meta(ItemMeta::Default(meta));
        println!("{}", item.stringified());
    }
}

#[macro_export]
macro_rules! declare_module {
    ($typ:ident,$ctor:path) => {
        extern "C" fn _plugin_ctor() -> *mut $crate::modules::Module {
            let ctor: fn() -> $typ = $ctor;
            let inst = ctor();
            let boxed = Box::new(inst);
            Box::into_raw(boxed)
        }
    }
}