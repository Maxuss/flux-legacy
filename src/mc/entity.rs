pub mod effect;
pub mod meta;
pub mod model;
pub mod types;

use crate::mc::world::WorldAccess;
use std::collections::HashMap;
use std::hash::Hash;
use std::io::Write;

use crate::nbt;
use crate::nbt::NbtTag;
use crate::prelude::{EntityMeta, EntityType, Identified};

pub trait IntoSelector: Clone {
    fn selector(&self) -> String;
}

impl<S> IntoSelector for S
where
    S: Into<String> + Clone,
{
    fn selector(&self) -> String {
        Clone::clone(self).into()
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Selector {
    AllEntities,
    AllPlayers,
    NearestPlayer,
    RandomPlayer,
    Executor,
}

impl IntoSelector for Selector {
    fn selector(&self) -> String {
        match *self {
            Selector::AllEntities => "@e",
            Selector::AllPlayers => "@a",
            Selector::NearestPlayer => "@p",
            Selector::RandomPlayer => "@r",
            Selector::Executor => "@c",
        }
        .to_string()
    }
}

#[derive(Debug, Clone)]
pub struct FullSelector<S>(Selector, HashMap<S, S>);

impl<S> FullSelector<S>
where
    S: Into<String> + Eq + Hash,
{
    pub fn new<const N: usize>(selector: Selector, params: [(S, S); N]) -> Self {
        Self(selector, HashMap::from(params))
    }
}

impl<S> IntoSelector for FullSelector<S>
where
    S: Into<String> + Clone,
{
    fn selector(&self) -> String {
        let mut buf = String::new();
        let mut iter = self.1.iter().peekable();
        if iter.peek().is_some() {
            buf.push_str("[");
            while let Some((k, v)) = iter.next() {
                let ks: String = Clone::clone(k).into();
                let vs: String = Clone::clone(v).into();
                buf.push_str(format!("{}={}", ks, vs).as_str());
                if iter.peek().is_some() {
                    buf.push_str(",");
                } else {
                    buf.push_str("]");
                }
            }
        }
        format!("{}{}", self.0.selector(), buf)
    }
}

#[derive(Debug, Clone)]
pub struct AttributeModifier {
    attribute: Attribute,
    amount: f64,
    operation: i32,
}

impl AttributeModifier {
    pub fn new(attribute: Attribute, operation: AttributeOperation, amount: f64) -> Self {
        Self {
            attribute,
            amount,
            operation: operation.into(),
        }
    }
}

impl Into<NbtTag> for AttributeModifier {
    fn into(self) -> NbtTag {
        let attr = self.attribute;
        let amount = self.amount;
        let oper = self.operation;
        NbtTag::Compound(nbt! {
            AttributeName: attr,
            Amount: amount,
            Operation: oper
        })
    }
}

pub enum AttributeOperation {
    Add,
    MultiplyBase,
    Multiply,
}

impl Into<i32> for AttributeOperation {
    fn into(self) -> i32 {
        match self {
            AttributeOperation::Add => 0,
            AttributeOperation::MultiplyBase => 1,
            AttributeOperation::Multiply => 2,
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
    ZombieSpawnReinforcements,
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
            Attribute::ZombieSpawnReinforcements => "zombie.spawn_reinforcements",
        }
        .to_string()
    }
}

impl Into<NbtTag> for Attribute {
    fn into(self) -> NbtTag {
        NbtTag::String(self.to_string())
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Entity {
    ty: EntityType,
    pub(crate) meta: EntityMeta,
    pub(crate) id: u64,
}

impl Entity {
    pub fn new(ty: EntityType) -> Self {
        Self {
            ty,
            meta: EntityMeta::new(ty),
            id: rand::random(),
        }
    }

    pub fn get_type(&self) -> EntityType {
        self.ty
    }

    pub fn provide_meta<F: FnOnce() -> EntityMeta>(&mut self, generator: F) {
        self.meta = generator()
    }

    pub fn modify_meta<F: FnOnce(&mut EntityMeta) -> EntityMeta>(&mut self, modifier: F) {
        self.meta = modifier(&mut self.meta)
    }

    pub fn save<W: Write>(&mut self, world: &mut WorldAccess<W>) {
        let sel = format!("@e[tag=fluxd{}]", self.id);
        world.write_line(format!(
            "execute if entity {} as {} run data merge entity @s {}",
            sel,
            sel,
            self.meta.stringified()
        ))
    }
}
