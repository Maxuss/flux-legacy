use convert_case::{Case, Casing};
use crate::chat::Component;
use crate::utils::{Either, Positive, Vec3D, Vec3I};
use crate::mc::block::Location;
use crate::nbt::NbtTag;
use crate::mc::entity::effect::PotionEffect;
use uuid::Uuid;
use crate::mc::entity::Attribute;
use crate::{__meta_struct, nbt};
use crate::mc::{Identified, Identifier};
use crate::mc::item::MetaContainer;
use crate::nbt::NbtWriter;
use crate::utils::GeneralColor;
use crate::mc::item::Slot;
use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum EntityMeta {
    Standard()
}

impl Into<NbtTag> for EntityMeta {
    fn into(self) -> NbtTag {
        //todo
        NbtTag::Compound(nbt!())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct EntityRotation {
    yaw: f32,
    pitch: f32
}

impl EntityRotation {
    pub fn new(yaw: f32, pitch: f32) -> Self {
        Self {
            yaw, pitch
        }
    }
}

impl Into<NbtTag> for EntityRotation {
    fn into(self) -> NbtTag {
        NbtTag::List(vec![NbtTag::Float(self.yaw), NbtTag::Float(self.pitch)])
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ArmorDropChances {
    feet: f32,
    legs: f32,
    chest: f32,
    head: f32
}

impl ArmorDropChances {
    pub fn new(feet: Option<f32>, legs: Option<f32>, chest: Option<f32>, head: Option<f32>) -> Self {
        Self {
            feet: feet.unwrap_or(1.0),
            legs: legs.unwrap_or(1.0),
            chest: chest.unwrap_or(1.0),
            head: head.unwrap_or(1.0)
        }
    }
}

impl Into<NbtTag> for ArmorDropChances {
    fn into(self) -> NbtTag {
        NbtTag::List(vec![NbtTag::Float(self.feet), NbtTag::Float(self.legs), NbtTag::Float(self.chest), NbtTag::Float(self.head)])
    }
}

#[derive(Debug, Clone)]
pub struct Equipment {
    feet: ItemStack,
    legs: ItemStack,
    chest: ItemStack,
    head: ItemStack
}

impl Equipment {
    pub fn new(feet: Option<ItemStack>, legs: Option<ItemStack>, chest: Option<ItemStack>, head: Option<ItemStack>) -> Self {
        Self {
            feet: feet.unwrap_or_else(ItemStack::empty_stack),
            legs: legs.unwrap_or_else(ItemStack::empty_stack),
            chest: chest.unwrap_or_else(ItemStack::empty_stack),
            head: head.unwrap_or_else(ItemStack::empty_stack)
        }
    }
}

impl Into<NbtTag> for Equipment {
    fn into(self) -> NbtTag {
        NbtTag::List(vec![self.feet.into(), self.legs.into(), self.chest.into(), self.head.into()])
    }
}

#[derive(Debug, Copy, Clone)]
pub struct EntityAttribute {
    attr: Attribute,
    base: f64
}

impl EntityAttribute {
    pub fn new(attribute: Attribute, value: f64) -> Self {
        Self {
            attr: attribute,
            base: value
        }
    }
}

impl Into<NbtTag> for EntityAttribute {
    fn into(self) -> NbtTag {
        let name = self.attr.to_string();
        let base = self.base;
        NbtTag::Compound(nbt! {
            Name: name,
            Base: base
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct HandDropChances {
    main_hand: f32,
    off_hand: f32
}

impl HandDropChances {
    pub fn new(main_hand: Option<f32>, off_hand: Option<f32>) -> Self {
        Self {
            main_hand: main_hand.unwrap_or(1.0),
            off_hand: off_hand.unwrap_or(1.0)
        }
    }
}

impl Into<NbtTag> for HandDropChances {
    fn into(self) -> NbtTag {
        NbtTag::List(vec![NbtTag::Float(self.main_hand), NbtTag::Float(self.off_hand)])
    }
}

#[derive(Debug, Clone)]
pub struct HandItems {
    main_hand: ItemStack,
    off_hand: ItemStack
}

impl HandItems {
    pub fn new(main_hand: Option<ItemStack>, off_hand: Option<ItemStack>) -> Self {
        Self {
            main_hand: main_hand.unwrap_or_else(ItemStack::empty_stack),
            off_hand: off_hand.unwrap_or_else(ItemStack::empty_stack)
        }
    }
}

impl Into<NbtTag> for HandItems {
    fn into(self) -> NbtTag {
        NbtTag::List(vec![self.main_hand.into(), self.off_hand.into()])
    }
}


macro_rules! entities {
    (
        $(
            $name:ident {
                $(
                $stored_name:ident by $field_name:ident: $typ:ident $(<$generic:ident>)?
                ),* $(,)*
            }
        );* $(;)*
    ) => {
        $crate::__meta_struct! {
            $(
                $name {
                    Air by air: i16,
                    CustomName by custom_name: Component,
                    CustomNameVisible by custom_name_visible: bool,
                    FallDistance by fall_distance: f32,
                    Fire by fire_ticks: i16,
                    Glowing by is_glowing: bool,
                    HasVisualFire by has_visual_fire: bool,
                    Invulnerable by invulnerable: bool,
                    Motion by motion: Vec3D,
                    NoGravity by ignore_gravity: bool,
                    OnGround by on_ground: bool,
                    Passengers by passengers: Vec<EntityMeta>,
                    PortalCooldown by portal_cooldown: i32,
                    Pos by position: Vec3D,
                    Rotation by rotation: EntityRotation,
                    Silent by silent: bool,
                    Tags by tags: Vec<String>,
                    TicksFrozen by freezing_ticks: i32,
                    UUID by uuid: Uuid,
                    $(
                        $stored_name by $field_name: $typ $(<$generic>)?
                    ),*
                }
            );*
        }
    };
}

pub type UuidOrLocation = Either<Uuid, Location>;

macro_rules! mobs {
    (
        $(
            $name:ident {
                $(
                $mcname:ident by $fname:ident: $typ:ident $(<$generic:ident>)?
                ),* $(,)*
            }
        );* $(;)*
    ) => {
        entities! {
            $(
                $name {
                    AbsorptionAmount by absorption_amount: i32,
                    ActiveEffects by active_effects: Vec<PotionEffect>,
                    ArmorDropChances by armor_drop_chances: ArmorDropChances,
                    ArmorItems by equipment: Equipment,
                    Attributes by attributes: Vec<EntityAttribute>,
                    CanPickUpLoot by can_pick_up_loot: bool,
                    DeathLootTable by death_loot_table: Identifier,
                    DeathLootTableSeed by death_loot_table_seed: i64,
                    DeathTime by time_dead: i16,
                    FallFlying by is_gliding: bool,
                    Health by health: f32,
                    HurtTime by time_hurt: i16,
                    HandDropChances by hand_drop_chances: HandDropChances,
                    HandItems by hand_items: HandItems,
                    Leash by leashed_entity: UuidOrLocation,
                    LeftHanded by is_left_handed: bool,
                    NoAI by no_ai: bool,
                    PersistenceRequired by dont_despawn: bool,
                    SleepingX by sleeping_x: i32,
                    SleepingY by sleeping_y: i32,
                    SleepingZ by sleeping_z: i32,
                    Team by team: String,
                    $(
                        $mcname by $fname: $typ $(<$generic>)?
                    ),*
                }
            );*
        }
    };
}

macro_rules! breedable {
    (
        $(
            $name:ident {
                $(
                $mcname:ident by $fname:ident: $typ:ident $(<$generic:ident>)?
                ),* $(,)*
            }
        );* $(;)*
    ) => {
        mobs! {
            $(
                $name {
                    Age by age: i32,
                    ForcedAge by forced_age: i32,
                    InLove by love_ticks: i32,
                    LoveCause by in_love_with: Uuid,
                    $(
                        $mcname by $fname: $typ $(<$generic>)?
                    ),*
                }
            );*
        }
    };
}

macro_rules! horse {
    (
        $(
            $name:ident {
                $(
                $mcname:ident by $fname:ident: $typ:ident $(<$generic:ident>)?
                ),* $(,)*
            }
        );* $(;)*
    ) => {
        breedable! {
            $(
                $name {
                    ArmorItem by armor_item: ItemStack,
                    Bred by was_bred: bool,
                    EatingHaystack by is_eating: bool,
                    Owner by owner: Uuid,
                    SaddleItem by saddle_item: ItemStack,
                    Tame by is_tamed: bool,
                    Temper by temper: i32,
                    $(
                        $mcname by $fname: $typ $(<$generic>)?
                    ),*
                }
            );*
        }
    };
}

entities! {
    GeneralEntity { };
}

mobs! {
    // GeneralLivingEntity { };

    Allay {
        CanDuplicate by can_duplicate: bool,
        DuplicationCooldown by duplication_cooldown: i64,
        Inventory by inventory: Vec<ItemStack>,
        listener by vibration_listener: AllayVibrationListener,
    };

    Bat {
        BatFlags by is_sleeping: bool
    };

    Cod {
        FromBucket by from_bucket: bool
    };

    Creeper {
        ExplosionRadius by explosion_radius: u8,
        Fuse by fuse_timer: i16,
        ignited by is_ignited: bool,
        powered by is_charged: bool
    };

    Dolphin {
        CanFindTreasure by can_find_treasure: bool,
        GotFish by got_player_fish: bool,
        TreasurePosX by treasure_x: i32,
        TreasurePosY by treasure_y: i32,
        TreasurePosZ by treasure_z: i32
    };

    GeneralZombie {
        CanBreakDoors by can_break_doors: bool,
        DrownedConversionTimes by until_becomes_drowned: i32,
        InWaterTime by ticks_in_water: i32,
        IsBaby by is_baby: bool,
    };

    EnderDragon {
        DragonPhase by battle_phase: DragonPhase,
    };

    Enderman {
        AngerTime by anger_ticks: i32,
        AngryAt by angry_at: Uuid,
        // todo: the carried block state
    }
}

breedable! {
    GeneralBreedableEntity { };

    Axolotl {
        FromBucket by is_from_bucket: bool,
        Variant by color: AxolotlColor,
    };

    Bee {
        AngerTimer by angry_ticks: i32,
        AngryAt by angry_at: Uuid,
        CannotEnterHiveTicks by ticks_until_can_enter_hive: i32,
        CropsGrownSincePollination by crops_grown: i32,
        FlowerPos by circling_location: Location,
        HasNectar by has_nectar: bool,
        HasStung by has_stung: bool,
        HivePos by hive_location: Location,
        TicksSincePollination by ticks_since_pollination: i32,
    };

    Cat {
        Owner by owner: Uuid,
        Sitting by is_sitting: bool,
        CollarColor by collar_color: GeneralColor,
        variant by variant: CatVariant
    };

    Chicken {
        EggLayTime by until_lays_egg: i32,
        IsChickenJockey by is_jockey: bool
    };
}

horse! {
    Donkey {
        ChestedHorse by has_chests: bool,
        Items by items: Vec<Slot>,
    };
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum DragonPhase {
    Circling,
    Strafing,
    PreparingToLand,
    Landing,
    TakingOff,
    BreathAttack,
    SearchingForPlayer,
    LandedRoar,
    ChargingPlayer,
    PreparingToDie,
    Hovering
}

impl Into<NbtTag> for DragonPhase {
    fn into(self) -> NbtTag {
        NbtTag::Int(self as i32)
    }
}

#[derive(Debug, Clone)]
pub enum CatVariant {
    Tabby,
    Black,
    Red,
    Siamese,
    BritishShorthair,
    Calico,
    Persian,
    Ragdoll,
    White,
    Jellie,
    AllBlack
}

impl ToString for CatVariant {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl Identified for CatVariant {
    fn id(&self) -> Identifier {
        let str = self.to_string();
        Identifier::minecraft(str.to_case(Case::Snake))
    }
}

impl Into<NbtTag> for CatVariant {
    fn into(self) -> NbtTag {
        NbtTag::String(self.id().to_string())
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum AxolotlColor {
    Pink,
    Brown,
    Gold,
    Cyan,
    Blue
}

impl Into<NbtTag> for AxolotlColor {
    fn into(self) -> NbtTag {
        NbtTag::Int(self as i32)
    }
}

pub type AllayVibrationSource = Either<AllayBlockVibrationSource, AllayEntityVibrationSource>;

#[derive(Debug, Copy, Clone)]
pub struct AllayBlockVibrationSource(Location);
#[derive(Debug, Clone)]
pub struct AllayEntityVibrationSource(Uuid, f32);

impl Into<NbtTag> for AllayBlockVibrationSource {
    fn into(self) -> NbtTag {
        let pos: Vec3I = self.0.into();
        NbtTag::Compound(nbt! {
            type: "block",
            pos: pos,
        })
    }
}

impl Into<NbtTag> for AllayEntityVibrationSource {
    fn into(self) -> NbtTag {
        let source_entity = self.0;
        let y_offset = self.1;
        NbtTag::Compound(nbt! {
            type: "entity",
            source_entity: source_entity,
            y_offset: y_offset
        })
    }
}

__meta_struct! {
    AllayVibrationListener {
        distance by distance: Positive<i32>,
        event by event: AllayVibrationEvent,
        event_delay by event_delay: Positive<i32>,
        event_distance by event_distance: Positive<i32>,
        range by event_range: Positive<i32>,
        source by source: AllayVibrationSource,
    };

    AllayVibrationEvent {
        distance by distance: Positive<i32>,
        game_event by game_event: Identifier,
        pos by position: Vec3D,
        projectile_owner by projectile_owner: Uuid,
        source by source_entity: Uuid,
    }
}