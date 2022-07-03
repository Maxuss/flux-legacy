use crate::chat::Component;
use crate::utils::Vec3D;
use crate::mc::block::Location;
use crate::nbt::NbtTag;
use uuid::Uuid;

pub enum EntityMeta {
    Standard()
}

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
            )*
        }
    };
}

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
                    AbsorptionAmount by absorption_amount: i32
                    $(
                        $mcname by $fname: $typ $(<$generic>)?
                    )*
                }
            )*
        }
    };
}

mobs! {
    example {

    }
}