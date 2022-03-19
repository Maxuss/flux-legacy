#![allow(non_snake_case)]

use crate::chat::Component;
use crate::mc::material::Material;
use crate::nbt::{NbtTag, NbtWriter};
use crate::mc::enchant::Enchantment;
use crate::nbt;

#[derive(Debug, Clone)]
pub struct ItemStack {
    mat: Material,
    amount: i8,
    display: ItemDisplay
}

pub trait MetaContainer {
    fn write_meta<W>(&mut self, writer: &mut W) -> anyhow::Result<()> where W: NbtWriter;
}

#[derive(Default, Debug, Clone)]
pub struct ItemDisplay {
    name: Option<Component>,
    lore: Option<Vec<Component>>
}

impl ItemDisplay {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(&mut self, name: Component) -> Self {
        self.name = Some(name);
        self.clone()
    }

    pub fn lore(&mut self, lore: Vec<Component>) -> Self {
        self.lore = Some(lore);
        self.clone()
    }
}

impl Into<NbtTag> for ItemDisplay {
    fn into(self) -> NbtTag {
        let name = self.name;
        let lore = self.lore;
        NbtTag::Compound(nbt! {
            Name: name,
            Lore: lore
        })
    }
}

macro_rules! meta_impl {
    ($(
    $name:ident with $bname:ident {
        $(
        $field:ident: $typ:ident $(<$generic:ident>)?
        ),* $(,)*
    }
    );* $(;)*) => {
        $(
        #[derive(Debug, Clone)]
        pub struct $name {
            $(
            $field:Option<$typ $(<$generic>)?>,
            )*
        }

        #[derive(Debug, Clone)]
        pub struct $bname {
            $(
            $field:Option<$typ $(<$generic>)?>,
            )*
        }

        impl $name {
            pub fn builder() -> $bname {
                $bname {
                    $(
                    $field: None,
                    )*
                }
            }

            $(
            pub fn $field(&mut self, value: $typ $(<$generic>)*) -> Self {
                self.$field = Some(value);
                self.clone()
            }
            )*
        }

        impl MetaContainer for $name {
            fn write_meta<W>(&mut self, writer: &mut W) -> anyhow::Result<()> where W: NbtWriter {
                $(
                    writer.write_tag(Some(stringify!($field).to_string()), self.$field.to_owned().into())?;
                )*
                Ok(())
            }
        }
        )*
    }
}

meta_impl! {
    DefaultMeta with DefaultMetaBuilder {
        Enchantments: Vec<Enchantment>,
        display: ItemDisplay,
    };
}