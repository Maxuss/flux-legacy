use crate::__meta_struct;
use crate::mc::item::MetaContainer;
use crate::nbt::{NbtTag, NbtWriter};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Effect {
    Speed = 1,
    Slowness,
    Haste,
    MiningFatigue,
    Strength,
    InstantHealth,
    InstantDamage,
    JumpBoost,
    Nausea,
    Regeneration,
    Resistance,
    FireResistance,
    WaterBreathing,
    Invisibility,
    Blindness,
    NightVision,
    Hunger,
    Weakness,
    Poison,
    Wither,
    HealthBoost,
    Absorption,
    Saturation,
    Glowing,
    Levitation,
    Luck,
    Unluck,
    SlowFalling,
    ConduitPower,
    DolphinsGrace,
    BadOmen,
    HeroOfTheVillage,
    Darkness,
}

impl Into<NbtTag> for Effect {
    fn into(self) -> NbtTag {
        NbtTag::Byte(self as i8)
    }
}

__meta_struct! {
    PotionEffect {
        Ambient by is_ambient: bool,
        Amplifier by level: u8,
        Duration by duration: i32,
        HiddenEffect by next_effect: Box<PotionEffect>,
        Id by effect: Effect,
        ShowIcon by show_icon: bool,
        ShowParticles by show_particles: bool
    }
}
