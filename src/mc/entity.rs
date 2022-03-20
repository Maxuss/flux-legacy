use crate::nbt;
use crate::nbt::NbtTag;

#[derive(Debug, Clone)]
pub struct AttributeModifier {
    attribute: Attribute,
    amount: i32,
    operation: i32
}

impl AttributeModifier {
    pub fn new(attribute: Attribute, operation: AttributeOperation, amount: i32) -> Self {
        Self {
            attribute,
            amount,
            operation: operation.into()
        }
    }
}

impl Into<NbtTag> for AttributeModifier {
    fn into(self) -> NbtTag {
        let attr = self.attribute;
        let amount = self.amount;
        let oper = self.operation;
        NbtTag::Compound(nbt! {
            attribute: attr,
            amount: amount,
            operation: oper
        })
    }
}

pub enum AttributeOperation {
    Add,
    MultiplyBase,
    Multiply
}

impl Into<i32> for AttributeOperation {
    fn into(self) -> i32 {
        match self {
            AttributeOperation::Add => 0,
            AttributeOperation::MultiplyBase => 1,
            AttributeOperation::Multiply => 2
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Attribute {
    // generic
    MaxHealth,
    FollowRange,
    KnockbackResistance,
    MovementSpeed,
    AttackDamage,
    Armor,
    ArmorToughness,
    AttackKnockback,
    AttackSpeed,
    Luck,

    // horse
    HorseJumpStrength,

    // flying (bees + parrots)
    FlyingSpeed,

    // zombies
    ZombieSpawnReinforcements
}

impl ToString for Attribute {
    fn to_string(&self) -> String {
        match *self {
            Attribute::MaxHealth => "generic.max_health",
            Attribute::FollowRange => "generic.follow_range",
            Attribute::KnockbackResistance => "generic.knockback_resistance",
            Attribute::MovementSpeed => "generic.movement_speed",
            Attribute::AttackDamage => "generic.attack_damage",
            Attribute::Armor => "generic.armor",
            Attribute::ArmorToughness => "generic.armor_toughness",
            Attribute::AttackKnockback => "generic.attack_knockback",
            Attribute::AttackSpeed => "generic.attack_speed",
            Attribute::Luck => "generic.luck",
            Attribute::HorseJumpStrength => "horse.jump_strength",
            Attribute::FlyingSpeed => "generic.flying_speed",
            Attribute::ZombieSpawnReinforcements => "zombie.spawn_reinforcements"
        }.to_string()
    }
}

impl Into<NbtTag> for Attribute {
    fn into(self) -> NbtTag {
        NbtTag::String(self.to_string())
    }
}