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
use crate::mc::entity::types::EntityType;
use crate::mc::entity::effect::Effect;
use crate::snbt::StringNbtWriter;
use crate::utils::Vec3F;

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

macro_rules! raid_entity {
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
                    CanJoinRaid by can_join_raid: bool,
                    PatrolLeader by is_patrol_leader: bool,
                    Patrolling by is_patrolling: bool,
                    PatrolTarget by patrol_target: Location,
                    RaidId by raid_id: i32,
                    Wave by wave_spawned: i32,
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

macro_rules! projectiles {
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
                    HasBeenShot by has_been_shot: bool,
                    LeftOwner by left_owner_hitbox: bool,
                    Owner by owner: Uuid,
                    $(
                        $mcname by $fname: $typ $(<$generic>)?
                    ),*
                }
            );*
        }
    };
}

macro_rules! arrows {
    (
        $(
            $name:ident {
                $(
                $mcname:ident by $fname:ident: $typ:ident $(<$generic:ident>)?
                ),* $(,)*
            }
        );* $(;)*
    ) => {
        projectiles! {
            $(
                $name {
                    crit by is_critical: bool,
                    damage by damage: f64,
                    life by life: i16,
                    pickup by pickup_status: ArrowStatus,
                    PierceLevel by pierces_left: u8,
                    shake by ticks_until_pick: u8,
                    ShotFromCrossbow by is_crossbow_origin: bool,
                    SoundEvent by sound: String,
                    $(
                        $mcname by $fname: $typ $(<$generic>)?
                    ),*
                }
            );*
        }
    };
}

macro_rules! minecarts {
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
                // todo: displayed block
                $name {
                    CustomDisplayTitle by display_custom_title: bool,
                    DisplayOffset by display_offset: i32,
                    $(
                        $mcname by $fname: $typ $(<$generic>)?
                    ),*
                }
            );*
        }
    };
}

macro_rules! meta_enum {
    (
        $(
        $($lh:ident)|* = $rh:ident
        ),* $(,)*
    ) => {
        #[derive(Debug, Clone)]
        pub enum EntityMeta {
            $(
            $rh($rh)
            ),*
        }

        impl EntityMeta {
            pub fn new(ty: EntityType) -> Self {
                use $crate::mc::entity::types::EntityType;
                match ty {
                    EntityType::Player => panic!("Can not generate entity meta for player!"),
                    $($(EntityType::$lh)|* => return EntityMeta::$rh($rh::new())),*
                }
            }
        }

        impl Into<NbtTag> for EntityMeta {
            fn into(self) -> NbtTag {
                use $crate::mc::entity::meta::EntityMeta::*;
                match(self) {
                    $( $rh(value)  => value.into() ),*
                }
            }
        }

        impl MetaContainer for EntityMeta {
            fn write_meta<W>(&mut self, writer: &mut W) -> anyhow::Result<()> where W: NbtWriter {
                use $crate::mc::entity::meta::EntityMeta::*;
                match self {
                    $( $rh(value)  => value.write_meta(writer) ),*
                }
            }

            fn tag(&self) -> NbtTag {
                Clone::clone(self).into()
            }
        }
    };
}

impl EntityMeta {
    pub fn stringified(&mut self) -> String {
        let mut buf = vec![];
        let mut writer = StringNbtWriter::new(&mut buf);

        self.write_meta(&mut writer).unwrap();

        let str = String::from_utf8(buf).unwrap();
        str
    }
}

impl CommandLike for EntityMeta {
    fn compile(&mut self) -> String where Self: Sized {
        self.stringified()
    }
}

meta_enum! {
    Allay = Allay,
    Axolotl = Axolotl,
    Bat = Bat,
    Bee = Bee,
    Cat = Cat,
    Chicken = Chicken,
    Cod = Cod,
    Creeper = Creeper,
    Dolphin = Dolphin,
    EnderDragon = EnderDragon,
    Enderman = Enderman,
    Endermite = Endermite,
    Evoker = Evoker,
    Fox = Fox,
    Frog = Frog,
    Ghast = Ghast,
    GlowSquid = GlowSquid,
    Goat = Goat,
    Hoglin = Hoglin,
    Horse = Horse,
    Illusioner = Illusioner,
    IronGolem = IronGolem,
    Llama | TraderLlama = Llama,
    Mooshroom = Mooshroom,
    Ocelot = Ocelot,
    Panda = Panda,
    Parrot = Parrot,
    Phantom = Phantom,
    Pig = Pig,
    Piglin = Piglin,
    PiglinBrute = PiglinBrute,
    Pillager = Pillager,
    PolarBear = PolarBear,
    Pufferfish = Pufferfish,
    Rabbit = Rabbit,
    Ravager = Ravager,
    Salmon = Salmon,
    Sheep = Sheep,
    Shulker = Shulker,
    Skeleton = Skeleton,
    SkeletonHorse = SkeletonHorse,
    SnowGolem = SnowGolem,
    Strider = Strider,
    Tadpole = Tadpole,
    TropicalFish = TropicalFish,
    Turtle = Turtle,
    Vex = Vex,
    Villager = Villager,
    Vindicator = Vindicator,
    WanderingTrader = WanderingTrader,
    Warden = Warden,
    Wither = Wither,
    Wolf = Wolf,
    Zoglin = Zoglin,
    ZombieVillager = ZombieVillager,
    ZombifiedPiglin = ZombifiedPiglin,

    Arrow = Arrow,
    FireworkRocket = FireworkRocket,
    ShulkerBullet = ShulkerBullet,
    SpectralArrow = SpectralArrow,
    Trident = Trident,

    ExperienceOrb = ExperienceOrb,
    Item = DroppedItem,

    Boat = Boat,
    ChestBoat = ChestBoat,

    Minecart = Minecart,
    ChestMinecart = ChestMinecart,
    CommandBlockMinecart = CommandBlockMinecart,
    FurnaceMinecart = FurnaceMinecart,
    HopperMinecart = HopperMinecart,
    SpawnerMinecart = SpawnerMinecart,
    TntMinecart = TntMinecart,

    FallingBlock = FallingBlock,
    Tnt = TntEntity,

    AreaEffectCloud = AreaEffectCloud,
    ArmorStand = ArmorStand,
    EndCrystal = EndCrystal,
    EvokerFangs = EvokerFangs,
    EyeOfEnder = EyeOfEnder,
    Marker = Marker,
    Painting = Painting,

    // other
    Mule | Donkey = GeneralDonkey,
    Blaze | CaveSpider | ElderGuardian
    | Giant | Guardian | Silverfish
    | Spider | Squid | Stray
    | WitherSkeleton = GeneralLivingEntity,
    Egg | EnderPearl | ExperienceBottle | Snowball | Potion = ThrownItem,
    DragonFireball | Fireball | SmallFireball | WitherSkull = GeneralFireball,
    LlamaSpit = GeneralProjectile,
    Witch = GeneralRaidEntity,
    Cow = GeneralBreedableEntity,
    Drowned | Zombie | Husk = GeneralZombie,
    MagmaCube | Slime = GeneralSlime,
    ZombieHorse = GeneralHorse,
    FishingBobber | LeashKnot | LightningBolt = GeneralEntity,
    GlowItemFrame | ItemFrame = ItemFrame,
}

arrows! {
    Arrow {
        CustomPotionEffects by effects: Vec<PotionEffect>,
        Potion by potion_name: Identifier,
        CustomPotionColor by potion_color: i32,
        Color by color: i32,
    };

    SpectralArrow {
        Duration by glow_duration: i32,
    };

    Trident {
        DealtDamage by collided_before: bool,
        Trident by trident_item: ItemStack,
    };
}

projectiles! {
    GeneralProjectile { };

    ThrownItem {
        Item by thrown_item: ItemStack,
    };

    Fireball {
        power by acceleration: Vec3D,
        ExplosionPower by explosion_power: u8,
        Item by fired_item: ItemStack,
    };

    FireworkRocket {
        FireworksItem by item_shot: ItemStack,
        Life by life_ticks: i32,
        LifeTime by until_explodes: i32,
        ShotAtAngle by angle_shot: u8,
    };

    ShulkerBullet {
        Steps by steps: i32,
        Target by target: Uuid,
        TXD by offset_x: f64,
        TXY by offset_y: f64,
        TXZ by offset_z: f64,
    };

    GeneralFireball {
        power by acceleration: Vec3D,
    };
}

entities! {
    GeneralEntity { };

    ExperienceOrb {
        Age by age: i16,
        Count by count: i32,
        Health by orb_health: i16,
        Value by value: i16,
    };

    DroppedItem {
        Health by item_health: i16,
        Age by age: i16,
        Item by item: ItemStack,
        Owner by owner: Uuid,
        PickupDelay by pickup_delay: i16,
        Thrower by dropped_person: Uuid,
    };

    Boat {
        Type by boat_wood: String,
    };

    ChestBoat {
        Type by boat_wood: String,
        Items by items: Vec<Slot>,
        LootTable by loot_table: Identifier,
        LootTableSeed by loot_table_seed: i64,
    };

    FallingBlock {
        // todo: block states & tile entity
        DropItem by drops_item: bool,
        FallHurtAmount by fall_hurt_amount: f32,
        FallHurtMax by fall_hurt_max: i32,
        HurtEntities by hurt_entities: bool,
        Time by life_ticks: i32,
    };

    TntEntity {
        Fuse by fuse: i16,
    };

    AreaEffectCloud {
        Age by age: i32,
        Color by color: i32,
        Duration by duration: i32,
        DurationOnUse by duration_on_use: i32,
        Effects by effects: Vec<PotionEffect>,
        Owner by owner: Uuid,
        Particle by displayed_particle: String,
        Potion by potion_name: String,
        Radius by field_radius: f32,
        RadiusOnUse by radius_on_use: f32,
        RadiusPerTick by radius_per_tick: f32,
        ReapplicationDelay by reapply_delay: i32,
        WaitTime by wait_time: i32,
    };

    EndCrystal {
        BeamTarget by beam_target: Location,
        ShowBottom by show_bottom: bool,
    };

    EvokerFangs {
        Owner by owner: Uuid,
        Warmup by warmup_ticks: i32,
    };

    EyeOfEnder {
        Item by thrown_item: ItemStack,
    };

    ItemFrame {
        Facing by facing: u8,
        TileX by tile_x: i32,
        TileY by tile_y: i32,
        TileZ by tile_z: i32,
        Fixed by is_fixed: bool,
        Invisible by is_invisible: bool,
        Item by stored_item: ItemStack,
        ItemDropChance by item_drop_chance: f32,
        ItemRotation by item_rotation: u8,
    };

    Marker {
        data by data: NbtTag,
    };

    Painting {
        Facing by facing: u8,
        TileX by tile_x: i32,
        TileY by tile_y: i32,
        TileZ by tile_z: i32,
        variant by variant: String,
    };
}

minecarts! {
    Minecart { };

    ChestMinecart {
        Items by items: Vec<Slot>,
        LootTable by loot_table: Identifier,
        LootTableSeed by loot_table_seed: i64,
    };

    CommandBlockMinecart {
        Command by command: String,
        LastOutput by last_output: String,
        SuccessCount by success_count: i32,
        TrackOutput by tracks_output: bool,
    };

    FurnaceMinecart {
        Fuel by fuel_ticks: i16,
        PushX by push_x: f64,
        PushZ by push_z: f64,
    };

    HopperMinecart {
        Items by items: Vec<Slot>,
        LootTable by loot_table: Identifier,
        LootTableSeed by loot_table_seed: i64,
        Enabled by is_enabled: bool,
        TransferCooldown by transfer_cooldown: i32,
    };

    SpawnerMinecart {
        Delay by spawn_delay: i16,
        MaxNearbyEntities by max_nearby_entities: i16,
        MaxSpawnDelay by max_spawn_delay: i16,
        MinSpawnDelay by min_spawn_delay: i16,
        RequiredPlayerRange by required_player_range: i16,
        SpawnCount by spawn_count: i16,
        SpawnData by next_entity_tag: NbtTag,
        SpawnRange by spawn_range: i16,
    };

    TntMinecart {
        TNTFuse by tnt_fuse: i32,
    };
}

mobs! {
    GeneralLivingEntity { };

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
    };

    Endermite {
        Lifetime by lifetime: i32,
    };

    Ghast {
        ExplosionPower by explosion_radius: u8,
    };

    GlowSquid {
        DarkTicksRemaining by until_starts_glowing: i32,
    };

    IronGolem {
        AngerTime by anger_ticks: i32,
        AngryAt by angry_at: Uuid,
        PlayerCreated by is_player_made: bool,
    };

    GeneralSlime {
        Size by size: Positive<i32>,
        wasOnGround by is_on_ground: i32,
    };

    Parrot {
        Owner by owner: Uuid,
        Sitting by is_sitting: bool,
        Variant by color: ParrotColor,
    };

    Phantom {
        AX by circling_x: i32,
        AY by circling_y: i32,
        AZ by circling_z: i32,
        Size by size: Positive<i32>,
    };

    Piglin {
        CannotHunt by cannot_hunt: bool,
        Inventory by items: Vec<ItemStack>,
        IsBaby by is_baby: bool,
        IsImmuneToZombification by is_zombification_immune: bool,
        TimeInOverworld by overworld_ticks: i32,
    };

    PiglinBrute {
        IsImmuneToZombification by is_zombification_immune: bool,
        TimeInOverworld by overworld_ticks: i32,
    };

    Pufferfish {
        FromBucket by is_from_bucket: bool,
        PuffState by puff_state: PuffState,
    };

    Salmon {
        FromBucket by is_from_bucket: bool,
    };

    Shulker {
        APX by approximate_x: i32,
        APY by approximate_y: i32,
        APZ by approximate_z: i32,
        AttachFace by attached_direction: u8,
        Color by color: GeneralColor,
        Peek by head_height: i8,
    };

    Skeleton {
        StrayConversionTime by ticks_until_stray: i32,
    };

    SnowGolem {
        Pumpkin by has_pumpkin: bool,
    };

    Tadpole {
        Age by age: i32,
        FromBucket by is_from_bucket: bool,
    };

    TropicalFish {
        FromBucket by is_from_bucket: bool,
        Variant by variant: i32,
    };

    Vex {
        BoundX by wander_x: i32,
        BoundY by wander_y: i32,
        BoundZ by wander_z: i32,
        LifeTicks by life_ticks: i32,
    };

    Warden {
        anger by anger: WardenAnger,
    };

    Wither {
        Invul by invul_ticks: i32,
    };

    Zoglin {
        IsBaby by is_baby: bool,
    };

    ZombieVillager {
        CanBreakDoors by can_break_doors: bool,
        DrownedConversionTimes by until_becomes_drowned: i32,
        InWaterTime by ticks_in_water: i32,
        IsBaby by is_baby: bool,
        Gossips by gossips: Vec<VillagerGossip>,
        Offers by offers: VillagerOffers,
        Xp by experience: i32,
        ConversionTime by conversion_ticks: i32,
        ConversionPlayer by converting_player: Uuid,
    };

    ZombifiedPiglin {
        CanBreakDoors by can_break_doors: bool,
        DrownedConversionTimes by until_becomes_drowned: i32,
        InWaterTime by ticks_in_water: i32,
        IsBaby by is_baby: bool,
        AngerTime by anger_ticks: i32,
        AngryAt by angry_at: Uuid,
    };

    ArmorStand {
        DisabledSlots by disabled_slots: i32,
        Invisible by is_invisible: bool,
        Marker by is_marker: bool,
        NoBasePlate by no_base_plate: bool,
        Pose by pose: StandPose,
        ShowArms by show_arms: bool,
        Small by is_small: bool,
    };


}

raid_entity! {
    GeneralRaidEntity { };

    Evoker {
        SpellTicks by ticks_until_spell: i32
    };

    Illusioner {
        SpellTicks by ticks_until_spell: i32
    };

    Pillager {
        Inventory by items: Vec<ItemStack>,
    };

    Ravager {
        AttackTick by attack_cooldown: i32,
        RoarTick by roar_cooldown: i32,
        StunTick by stun_cooldown: i32,
    };

    Vindicator {
        Johnny by is_johnny: bool,
    };
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

    Fox {
        Crouching by is_crouching: bool,
        Sitting by is_sitting: bool,
        Sleeping by is_sleeping: bool,
        Trusted by trusted_player: Vec<Uuid>,
        Type by color: FoxColor,
    };

    Frog {
        variant by color: FrogColor,
    };

    Goat {
        HasLeftHorn by has_left_horn: bool,
        HasRightHorn by has_right_horn: bool,
        IsScreamingGoat by can_scream: bool,
    };

    Hoglin {
        CannotBeHunted by do_piglins_ignore: bool,
        IsImmuneToZombification by is_zombification_immune: bool,
        TimeInOverworld by overworld_ticks: i32,
    };

    Llama {
        Bred by is_bred: bool,
        ChestedHorse by has_chests: bool,
        DecorItem by decor_item: ItemStack,
        DespawnDelay by despawn_delay: i32,
        EatingHaystack by is_eating: bool,
        Items by items: Vec<Slot>,
        Owner by owner: Uuid,
        Variant by color: LlamaColor,
        Strength by strength: i32,
        Tame by is_tamed: bool,
        Temper by temper: i32,
    };

    Mooshroom {
        EffectDuration by given_effect_duration: i32,
        EffectId by given_effect: Effect,
        Type by color: MooshroomColor,
    };

    Ocelot {
        Trusting by trusts_player: bool,
    };

    Panda {
        HiddenGene by hidden_gene: PandaGene,
        MainGene by main_gene: PandaGene
    };

    Pig {
        Saddle by has_saddle: bool,
    };

    PolarBear {
        AngerTime by anger_ticks: i32,
        AngryAt by angry_at: Uuid,
    };

    Rabbit {
        MoreCarrotTicks by carrot_ticks: i32,
        RabbitType by rabbit_type: RabbitType,
    };

    Sheep {
        Color by color: GeneralColor,
        Sheared by is_sheared: bool,
    };

    SkeletonHorse {
        SkeletonTrap by is_trapped: bool,
        SkeletonTrapTime by time_until_despawns: i32,
    };

    Strider {
        Saddle by has_saddle: bool,
    };

    Turtle {
        HasEgg by has_egg: bool,
        HomePosX by home_x: i32,
        HomePosY by home_y: i32,
        HomePosZ by home_z: i32,
        TravelPosX by travel_x: i32,
        TravelPosY by travel_y: i32,
        TravelPosZ by travel_z: i32,
    };

    Villager {
        Inventory by items: Vec<ItemStack>,
        LastRestock by last_restock: i64,
        LastGossipDelay by last_gossip: i64,
        RestocksToday by restocks_today: i32,
        Willing by wants_to_mate: bool,
        Gossips by gossips: Vec<VillagerGossip>,
        Offers by offers: VillagerOffers,
        Xp by experience: i32,
    };

    WanderingTrader {
        DespawnDelay by despawn_ticks: i32,
        Offers by offers: VillagerOffers,
        WanderTarget by wander_target: Location,
        Inventory by items: Vec<ItemStack>,
    };

    Wolf {
        AngerTime by anger_ticks: i32,
        AngryAt by angry_at: Uuid,
        Owner by owner: Uuid,
        Sitting by is_sitting: bool,
        CollarColor by collar_color: GeneralColor,
    };
}

horse! {
    GeneralHorse { };

    GeneralDonkey {
        ChestedHorse by has_chests: bool,
        Items by items: Vec<Slot>,
    };

    Horse {
        Variant by color: i32,
    };
}

pub fn fish_variant(pattern: FishPattern, body_color: GeneralColor, pattern_color: GeneralColor) -> i32 {
    return i32::from_be_bytes([if pattern.is_big() { 1 } else { 0 }, pattern.id(), body_color as u8, pattern_color as u8]);
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum FishPattern {
    Flopper,
    Stripey,
    Glitter,
    Blockfish,
    Betty,
    Clayfish,

    Kob,
    Sunstreak,
    Snooper,
    Dasher,
    Brinely,
    Spotty,
}

impl FishPattern {
    pub fn is_big(&self) -> bool {
        use FishPattern::*;
        match self {
            Flopper | Stripey | Glitter | Blockfish | Betty | Clayfish => true,
            _ => false
        }
    }

    pub fn id(&self) -> u8 {
        use FishPattern::*;
        match self {
            Flopper | Stripey | Glitter | Blockfish | Betty | Clayfish => *self as u8,
            _ => *self as u8 - 6
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum ArrowStatus {
    NoPickup,
    Pickup,
    CreativePickup
}

impl Into<NbtTag> for ArrowStatus {
    fn into(self) -> NbtTag {
        NbtTag::Byte(self as i8)
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum PuffState {
    Deflated,
    Halfway,
    Puffed
}

impl Into<NbtTag> for PuffState {
    fn into(self) -> NbtTag {
        NbtTag::Int(self as i32)
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum RabbitType {
    Brown,
    White,
    Black,
    BlackAndWhite,
    Gold,
    SaltAndPepper,
    KillerBunny = 99
}

impl Into<NbtTag> for RabbitType {
    fn into(self) -> NbtTag {
        NbtTag::Int(self as i32)
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum ParrotColor {
    Red,
    Blue,
    Green,
    Cyan,
    Gray
}

impl Into<NbtTag> for ParrotColor {
    fn into(self) -> NbtTag {
        NbtTag::Int(self as i32)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum PandaGene {
    Normal,
    Lazy,
    Worried,
    Playful,
    Brown,
    Weak,
    Aggressive
}

impl Into<NbtTag> for PandaGene {
    fn into(self) -> NbtTag {
        NbtTag::String(format!("{:?}", self).to_lowercase())
    }
}

#[derive(Debug, Copy, Clone)]
pub enum MooshroomColor {
    Red,
    Brown,
}

impl Into<NbtTag> for MooshroomColor {
    fn into(self) -> NbtTag {
        NbtTag::String(format!("{:?}", self).to_lowercase())
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum LlamaColor {
    Creamy,
    White,
    Brown,
    Gray
}

impl Into<NbtTag> for LlamaColor {
    fn into(self) -> NbtTag {
        NbtTag::Int(self as i32)
    }
}

pub fn horse_color(base: HorseColor, markings: HorseMarkings) -> i32 {
    (base as i32) + (markings as i32)
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum HorseColor {
    White,
    Creamy,
    Chestnut,
    Brown,
    Black,
    Gray,
    DarkBrown
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum HorseMarkings {
    None = 0,
    White = 256,
    WhiteField = 512,
    WhiteDots = 768,
    BlackDots = 1024
}

#[derive(Debug, Copy, Clone)]
pub enum FrogColor {
    Temperate,
    Warm,
    Cold
}

impl Into<NbtTag> for FrogColor {
    fn into(self) -> NbtTag {
        NbtTag::String(Identifier::minecraft(format!("{:?}", self).to_lowercase()).to_string())
    }
}


#[derive(Debug, Copy, Clone)]
pub enum FoxColor {
    Red,
    Snow
}

impl Into<NbtTag> for FoxColor {
    fn into(self) -> NbtTag {
        NbtTag::String(format!("{:?}", self).to_lowercase())
    }
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
    };

    VillagerGossip {
        Value by value: i32,
        Target by target_player: Uuid,
        Type by gossip_type: GossipType,
    };

    VillagerOffer {
        buy by buys: ItemStack,
        buyB by buys_secondary: ItemStack,
        demand by demand: i32,
        maxUses by max_uses: i32,
        priceMultiplier by price_multiplier: f32,
        rewardExp by rewards_exp: bool,
        sell by sells: ItemStack,
        specialPrice by extra_modifier: i32,
        uses by times_used: i32,
        xp by villager_exp: i32,
    };

    VillagerData {
        level by level: VillagerLevel,
        profession by profession: Identifier,
        type by villager_type: Identifier,
    };

    VillagerOffers {
        Recipes by offers: Vec<VillagerOffer>,
    };

    WardenAnger {
        suspects by suspects: Vec<WardenSuspect>,
    };

    WardenSuspect {
        anger by anger_level: i32,
        uuid by uuid: Uuid,
    };

    StandPose {
        Body by body: Vec3F,
        Head by head: Vec3F,
        LeftArm by left_arm: Vec3F,
        LeftLeg by left_leg: Vec3F,
        RightArm by right_arm: Vec3F,
        RightLeg by right_leg: Vec3F,
    };
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum VillagerLevel {
    Novice = 1,
    Apprentice,
    Journeyman,
    Expert,
    Master
}

impl Into<NbtTag> for VillagerLevel {
    fn into(self) -> NbtTag {
        NbtTag::Int(self as i32)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum GossipType {
    MajorNegative,
    MinorNegative,
    MajorPositive,
    MinorPositive,
    Trading
}

impl Into<NbtTag> for GossipType {
    fn into(self) -> NbtTag {
        NbtTag::String(format!("{:?}", self).to_case(Case::Snake))
    }
}