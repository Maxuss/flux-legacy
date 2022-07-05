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
    use std::time::Instant;
    use crate::chat::{Component, NamedColor};
    use crate::component;
    use crate::mc::commands::{GiveCommand, SummonCommand};
    use crate::mc::enchant::{Enchant, Enchantment};
    use crate::mc::entity::{Attribute, AttributeModifier, AttributeOperation, FullSelector, IntoSelector, Selector};
    use crate::mc::entity::meta::{ArmorStand, Equipment, HandItems, StandPose};
    use crate::mc::item::{DefaultMeta, FLAG_HIDE_ATTRIBUTES, FLAG_HIDE_DESTROY, FLAG_HIDE_DYED, FLAG_HIDE_ENCHANTMENTS, FLAG_HIDE_PLACE, FLAG_HIDE_UNBREAKABLE, SkullData, SkullMeta, SkullOwner};
    use crate::prelude::*;
    use crate::utils::{Keybind, Vec3F};

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

    #[test]
    fn test_summon_command() {
        let meta = ArmorStand::new()
            .equipment(Equipment::new(
                None,
                None,
                Some(Material::NetheriteChestplate.stack()),
                Some(Material::PlayerHead.stack().provide_meta(|| ItemMeta::Skull(
                    SkullMeta::new().skull_owner(SkullOwner::Base64(SkullData::new("eyJ0ZXh0dXJlcyI6eyJTS0lOIjp7InVybCI6Imh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvZTVlYjBiZDg1YWFkZGYwZDI5ZWQwODJlYWMwM2ZjYWRlNDNkMGVlODAzYjBlODE2MmFkZDI4YTYzNzlmYjU0ZSJ9fX0="))
                    ))))
            ))
            .hand_items(HandItems::new(
                Some(Material::NetheriteSword.stack()),
                None
            ))
            .pose(StandPose::new().right_arm(Vec3F(287.0, 0.0, 0.0)))
            .show_arms(true);
        let time = Instant::now();
        let mut cmd = SummonCommand::new(EntityType::ArmorStand, Some("~ ~ ~".into()), Some(EntityMeta::ArmorStand(meta)));
        let diff = (Instant::now() - time).as_micros();
        println!("Took {}mcs", diff);
        println!("{}", cmd.compile())
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
