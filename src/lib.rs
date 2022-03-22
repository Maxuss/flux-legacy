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
    use crate::snbt::StringNbtWriter;
    use std::collections::HashMap;
    use std::env::current_dir;
    use crate::chat::{Component, NamedColor};
    use crate::mc::commands::GiveCommand;
    use crate::mc::enchant::{Enchant, Enchantment};
    use crate::mc::entity::{FullSelector, IntoSelector, Selector};
    use crate::mc::item::DefaultMeta;
    use crate::prelude::*;

    #[test]
    fn test_items() {
        let mut item = ItemStack::new(Material::DiamondSword);
        let mut meta = DefaultMeta::new();
        meta.enchants(vec![Enchantment::new(Enchant::Sharpness, 5)]);
        meta.unbreakable(true);
        let mut display = ItemDisplay::new();
        display.name(Component::text("Amazing sword").color(NamedColor::Gold).bold(true));
        meta.display(display);
        item.meta(ItemMeta::Default(meta));
        println!("{}", item.stringified());
    }

    #[test]
    fn give_command() {
        let mut cmd = GiveCommand::builder().amount(None).item(ItemStack::new(Material::DiamondSword));
        println!("{}", cmd.compile())
    }

    #[test]
    fn test_selectors() {
        let mut sel = FullSelector::new(Selector::AllEntities, [("range", "50"), ("max", "1")]);
        println!("{}", sel.selector());
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