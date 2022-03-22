extern crate core;

pub mod chat;
pub mod macros;
pub mod mc;
pub mod modules;
pub mod nbt;
pub mod prelude;
pub mod snbt;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::chat::{Component, NamedColor};
    use crate::component;
    use crate::mc::commands::GiveCommand;
    use crate::mc::enchant::{Enchant, Enchantment};
    use crate::mc::entity::{Attribute, AttributeModifier, AttributeOperation, FullSelector, IntoSelector, Selector};
    use crate::mc::item::{DefaultMeta, FLAG_HIDE_ATTRIBUTES, FLAG_HIDE_DESTROY, FLAG_HIDE_DYED, FLAG_HIDE_ENCHANTMENTS, FLAG_HIDE_PLACE, FLAG_HIDE_UNBREAKABLE};
    use crate::prelude::*;
    use crate::utils::Keybind;

    #[test]
    fn test_items() {
        let mut item = ItemStack::new(Material::DiamondSword, None);
        let mut meta = DefaultMeta::new();
        meta.enchants(vec![Enchantment::new(Enchant::Sharpness, 5)]);
        meta.unbreakable(true);
        let mut display = ItemDisplay::new();
        display.name(
            Component::text("Amazing sword")
                .color(NamedColor::Gold)
                .bold(true),
        );
        meta.display(display);
        item.meta(ItemMeta::Default(meta));
        println!("{}", item.stringified());
    }

    #[test]
    fn give_command() {
        let mut item = ItemStack::new(
            Material::DiamondChestplate, None);
        item.meta(ItemMeta::Default(
            DefaultMeta::new()
                .display(
                    ItemDisplay::new()
                        .name(
                            Component::text("Epic Chestplate")
                                .color(NamedColor::Gold)
                                .italic(false))
                        .lore(
                            vec![
                                Component::text("Strength: ")
                                    .color(NamedColor::Gray)
                                    .italic(false)
                                    .append(Component::text("+10").color(NamedColor::Green)),
                                Component::text(""),
                                Component::text("Press ")
                                    .color(NamedColor::Gray)
                                    .append(Component::keybind(Keybind::Attack).color(NamedColor::Green))
                                    .append(Component::text(" to attack!").color(NamedColor::Gray))
                                    .italic(false)
                            ]))
                .unbreakable(true)
                .hide_flags(
                    FLAG_HIDE_DYED | FLAG_HIDE_ATTRIBUTES |
                        FLAG_HIDE_DESTROY | FLAG_HIDE_PLACE |
                        FLAG_HIDE_ENCHANTMENTS | FLAG_HIDE_UNBREAKABLE)
                .attributes(vec![AttributeModifier::new(Attribute::MovementSpeed, AttributeOperation::Multiply, 1.23)])
                .enchants(vec![Enchantment::new(Enchant::Protection, 4)])
        ));

        let mut cmd = GiveCommand::new("@p", item);
        println!("{}", cmd.compile())
    }

    #[test]
    fn test_selectors() {
        let sel = FullSelector::new(Selector::AllEntities, [("range", "50"), ("max", "1")]);
        println!("{}", sel.selector());
    }

    #[test]
    fn component_macros() {
        let comp = component! { @0xff0000 bold italic "Red, Bold, and Italic " & !bold "just red and italic" };
        println!("{}", comp.to_string())
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
    };
}
