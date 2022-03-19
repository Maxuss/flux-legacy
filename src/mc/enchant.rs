use convert_case::{Case, Casing};
use crate::mc::{Identifiable, Identifier};
use crate::nbt;
use crate::nbt::NbtTag;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Enchant {
    Protection,
    FireProtection,
    FeatherFalling,
    BlastProtection,
    ProjectileProtection,
    Respiration,
    AquaAffinity,
    Thorns,
    DepthStrider,
    FrostWalker,
    BindingCurse,
    SoulSpeed,
    Sharpness,
    Smite,
    BaneOfArthropods,
    Knockback,
    FireAspect,
    Looting,
    Sweeping,
    Efficiency,
    SilkTouch,
    Unbreaking,
    Fortune,
    Power,
    Punch,
    Flame,
    Infinity,
    LuckOfTheSea,
    Lure,
    Loyalty,
    Impaling,
    Riptide,
    Channeling,
    Multishot,
    QuickCharge,
    Piercing,
    Mending,
    VanishingCurse,
}

impl Into<NbtTag> for Enchant {
    fn into(self) -> NbtTag {
        NbtTag::String(self.id().to_string())
    }
}

impl ToString for Enchant {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl Identifiable for Enchant {
    fn id(&self) -> Identifier {
        let str = self.to_string();
        Identifier::minecraft(str.to_case(Case::Snake))
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Enchantment {
    typ: Enchant,
    lvl: i32
}

impl Enchantment {
    pub fn new(enchant: Enchant, level: i32) -> Self {
        Self {
            typ: enchant,
            lvl: level
        }
    }
}

impl Identifiable for Enchantment {
    fn id(&self) -> Identifier {
        self.typ.id()
    }
}

impl Into<NbtTag> for Enchantment {
    fn into(self) -> NbtTag {
        let id = self.id();
        let lvl = self.lvl;
        NbtTag::Compound(nbt! {
            id: id,
            lvl: lvl
        })
    }
}