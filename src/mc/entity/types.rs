use crate::mc::{Identified, Identifier};
use crate::nbt::NbtTag;
use crate::prelude::EntityMeta;
use convert_case::{Case, Casing};

#[repr(u8)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum EntityType {
    Dynamic,
    Allay,
    Frog,
    Tadpole,
    AreaEffectCloud,
    ArmorStand,
    Arrow,
    Axolotl,
    Bat,
    Bee,
    Blaze,
    Boat,
    Cat,
    CaveSpider,
    Chicken,
    Cod,
    Cow,
    Creeper,
    ChestBoat,
    Dolphin,
    Donkey,
    DragonFireball,
    Drowned,
    ElderGuardian,
    EndCrystal,
    EnderDragon,
    Enderman,
    Endermite,
    Evoker,
    EvokerFangs,
    ExperienceOrb,
    EyeOfEnder,
    FallingBlock,
    FireworkRocket,
    Fox,
    Ghast,
    Giant,
    GlowItemFrame,
    GlowSquid,
    Goat,
    Guardian,
    Hoglin,
    Horse,
    Husk,
    Illusioner,
    IronGolem,
    Item,
    ItemFrame,
    Fireball,
    LeashKnot,
    LightningBolt,
    Llama,
    LlamaSpit,
    MagmaCube,
    Marker,
    Minecart,
    ChestMinecart,
    CommandBlockMinecart,
    FurnaceMinecart,
    HopperMinecart,
    SpawnerMinecart,
    TntMinecart,
    Mule,
    Mooshroom,
    Ocelot,
    Painting,
    Panda,
    Parrot,
    Phantom,
    Pig,
    Piglin,
    PiglinBrute,
    Pillager,
    PolarBear,
    Tnt,
    Pufferfish,
    Rabbit,
    Ravager,
    Salmon,
    Sheep,
    Shulker,
    ShulkerBullet,
    Silverfish,
    Skeleton,
    SkeletonHorse,
    Slime,
    SmallFireball,
    SnowGolem,
    Snowball,
    SpectralArrow,
    Spider,
    Squid,
    Stray,
    Strider,
    Egg,
    EnderPearl,
    ExperienceBottle,
    Potion,
    Trident,
    TraderLlama,
    TropicalFish,
    Turtle,
    Vex,
    Villager,
    Vindicator,
    WanderingTrader,
    Warden,
    Witch,
    Wither,
    WitherSkeleton,
    WitherSkull,
    Wolf,
    Zoglin,
    Zombie,
    ZombieHorse,
    ZombieVillager,
    ZombifiedPiglin,
    Player,
    FishingBobber,
}

impl EntityType {
    pub fn create_meta(&self) -> EntityMeta {
        EntityMeta::new(*self)
    }
}

impl ToString for EntityType {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl Identified for EntityType {
    fn id(&self) -> Identifier {
        Identifier::minecraft(self.to_string().to_case(Case::Snake))
    }
}

impl Into<NbtTag> for EntityType {
    fn into(self) -> NbtTag {
        NbtTag::String(self.id().to_string())
    }
}
