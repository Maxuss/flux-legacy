#![allow(non_snake_case)]

use uuid::Uuid;
use crate::chat::Component;
use crate::mc::material::Material;
use crate::nbt::{NbtTag, NbtWriter};
use crate::mc::enchant::Enchantment;
use crate::mc::entity::AttributeModifier;
use crate::mc::{Identifiable, Identifier};
use crate::nbt;
use crate::snbt::StringNbtWriter;

#[derive(Debug, Clone)]
pub struct ItemStack {
    mat: Material,
    meta: ItemMeta,
    amount: i8
}

impl ItemStack {
    pub fn new(mat: Material, amount: Option<i8>) -> Self {
        Self {
            mat,
            meta: ItemMeta::Default(DefaultMeta::new()),
            amount: amount.unwrap_or_else(|| { 1 })
        }
    }

    pub fn meta(&mut self, meta: ItemMeta) {
        self.meta = meta;
    }

    pub fn stringified(&mut self) -> String {
        let mut buf = vec![];
        let mut str = StringNbtWriter::new(&mut buf);
        self.meta.write_meta(&mut str).unwrap();
        let str = String::from_utf8(buf).unwrap();

        format!("{mat}{meta} {amount}", mat=self.mat.id().to_string(), meta=str, amount=self.amount)
    }
}

#[derive(Debug, Clone)]
pub enum ItemMeta {
    Default(DefaultMeta),
    Skull(SkullMeta)
}

impl MetaContainer for ItemMeta {
    fn write_meta<W>(&mut self, writer: &mut W) -> anyhow::Result<()> where W: NbtWriter {
        match self {
            ItemMeta::Default(m) => m.write_meta(writer),
            ItemMeta::Skull(m) => m.write_meta(writer)
        }
    }
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

#[derive(Debug, Clone, PartialEq)]
pub enum SkullOwner {
    Username(String),
    Base64(SkullData)
}

impl Into<NbtTag> for SkullOwner {
    fn into(self) -> NbtTag {
        match self {
            SkullOwner::Username(str) => str.into(),
            SkullOwner::Base64(data) => data.into()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SkullData {
    id: Uuid,
    name: String,
    texture: String
}

impl SkullData {
    pub fn new(texture: String) -> Self {
        let id = Uuid::new_v4();
        let rand_bytes: [u8; 32] = rand::random();
        let name = base64::encode(rand_bytes);
        Self {
            id,
            name,
            texture
        }
    }
}

impl Into<NbtTag> for SkullData {
    fn into(self) -> NbtTag {
        let id = self.id;
        let name = self.name;
        let texture = self.texture;
        NbtTag::Compound(nbt! {
            Id: id,
            Name: name,
            Properties: {
                textures: [
                    {
                        Value: texture
                    }
                ]
            }
        })
    }
}

macro_rules! meta_impl {
    (
        $(
            $(extended $defname:ident {
                $(
                $def_field:ident by $def_byname:ident: $def_typ:ident $(<$def_generic:ident>)?
                ),* $(,)*
            })?

            $(unique $name:ident {
                $(
                $field:ident by $byname:ident: $typ:ident $(<$generic:ident>)?
                ),* $(,)*
            })?
        );* $(;)*
    ) => {
        $(
            $(
                meta_impl! {
                    unique $defname {
                        Enchantments by enchants: Vec<Enchantment>,
                        display by display: ItemDisplay,
                        AttributeModifiers by attributes: Vec<AttributeModifier>,
                        Unbreakable by unbreakable: bool,
                        HideFlags by hide_flags: i32,
                        CanDestroy by can_destroy: Vec<Identifier>,
                        PickupDelay by pickup_delay: i32,
                        Age by age: i16,
                        $(
                            $def_field by $def_byname: $def_typ $(<$def_generic>)?
                        )*
                    }
                }
            )?
            $(
                #[derive(Debug, Clone)]
                pub struct $name {
                    $(
                    $byname:Option<$typ $(<$generic>)?>,
                    )*
                }

                impl $name {
                    pub fn new() -> Self {
                        $name {
                            $(
                            $byname: None,
                            )*
                        }
                    }

                    $(
                    pub fn $byname(&mut self, value: $typ $(<$generic>)*) -> Self {
                        self.$byname = Some(value);
                        self.clone()
                    }

                    )*
                }

                impl MetaContainer for $name {
                    fn write_meta<W>(&mut self, writer: &mut W) -> anyhow::Result<()> where W: NbtWriter {
                        $(
                        let $byname: NbtTag = Clone::clone(&self.$byname).into();
                        )*
                        let comp = $crate::nbt::NbtTag::Compound($crate::nbt! {
                            $(
                            $field: $byname,
                            )*
                        });

                        writer.write_tag(None, comp)?;

                        Ok(())
                    }
                }
            )?
        )*
    }
}

meta_impl! {
    extended DefaultMeta { };
    extended SkullMeta {
        SkullOwner by skull_owner: SkullOwner,
    }
}

pub const FLAG_HIDE_ENCHANTMENTS: i32 =  0b000001;
pub const FLAG_HIDE_ATTRIBUTES: i32 =    0b000010;
pub const FLAG_HIDE_UNBREAKABLE: i32 =   0b000100;
pub const FLAG_HIDE_DESTROY: i32 =       0b001000;
pub const FLAG_HIDE_PLACE: i32 =         0b010000;
pub const FLAG_HIDE_DYED: i32 =          0b100000;