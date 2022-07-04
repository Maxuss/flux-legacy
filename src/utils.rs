use byteorder::{BigEndian, ReadBytesExt};
use uuid::Uuid;
use crate::nbt::NbtTag;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Keybind {
    Jump,
    Sneak,
    Sprint,
    StrafeLeft,
    StrafeRight,
    WalkBackward,
    WalkForward,
    Attack,
    PickBlock,
    Use,
    Drop,
    Hotbar1,
    Hotbar2,
    Hotbar3,
    Hotbar4,
    Hotbar5,
    Hotbar6,
    Hotbar7,
    Hotbar8,
    Hotbar9,
    Inventory,
    SwapOffHand,
    LoadToolbar,
    SaveToolbar,
    PlayerList,
    OpenChat,
    Command,
    OpenSocialInteractions,
    OpenAdvancements,
    HighlightSpectators,
    Screenshot,
    SmoothCamera,
    Fullscreen,
    ChangePerspective,
}

impl Keybind {
    pub fn key(&self) -> String {
        String::from(match *self {
            Keybind::Jump => "key.jump",
            Keybind::Sneak => "key.sneak",
            Keybind::Sprint => "key.sprint",
            Keybind::StrafeLeft => "key.left",
            Keybind::StrafeRight => "key.right",
            Keybind::WalkBackward => "key.back",
            Keybind::WalkForward => "key.forward",
            Keybind::Attack => "key.attack",
            Keybind::PickBlock => "key.pickItem",
            Keybind::Use => "key.use",
            Keybind::Drop => "key.drop",
            Keybind::Hotbar1 => "key.hotbar.1",
            Keybind::Hotbar2 => "key.hotbar.2",
            Keybind::Hotbar3 => "key.hotbar.3",
            Keybind::Hotbar4 => "key.hotbar.4",
            Keybind::Hotbar5 => "key.hotbar.5",
            Keybind::Hotbar6 => "key.hotbar.6",
            Keybind::Hotbar7 => "key.hotbar.7",
            Keybind::Hotbar8 => "key.hotbar.8",
            Keybind::Hotbar9 => "key.hotbar.9",
            Keybind::Inventory => "key.inventory",
            Keybind::SwapOffHand => "key.swapOffhand",
            Keybind::LoadToolbar => "key.loadToolbarActivator",
            Keybind::SaveToolbar => "key.saveToolbarActivator",
            Keybind::PlayerList => "key.playerList",
            Keybind::OpenChat => "key.chat",
            Keybind::Command => "key.command",
            Keybind::OpenSocialInteractions => "key.socialInteractions",
            Keybind::OpenAdvancements => "key.advancements",
            Keybind::HighlightSpectators => "key.spectatorOutlines",
            Keybind::Screenshot => "key.screenshot",
            Keybind::SmoothCamera => "key.smoothCamera",
            Keybind::Fullscreen => "key.fullscreen",
            Keybind::ChangePerspective => "key.togglePerspective",
        })
    }
}

pub fn escape(src: String) -> String {
    let mut escaped = String::with_capacity(src.len());
    for c in src.chars() {
        match c {
            '\r' => escaped += "\\r",
            '\n' => escaped += "\\n",
            '\t' => escaped += "\\t",
            '"' => escaped += "\\\"",
            '\\' => escaped += "\\",
            c => escaped.push(c),
        };
    }
    escaped
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3D(pub f64, pub f64, pub f64);

impl Into<NbtTag> for Vec3D {
    fn into(self) -> NbtTag {
        NbtTag::List(vec![NbtTag::Double(self.0), NbtTag::Double(self.1), NbtTag::Double(self.2)])
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3F(pub f32, pub f32, pub f32);

impl Into<NbtTag> for Vec3F {
    fn into(self) -> NbtTag {
        NbtTag::List(vec![NbtTag::Float(self.0), NbtTag::Float(self.1), NbtTag::Float(self.2)])
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vec2F(f32, f32);

impl Into<NbtTag> for Vec2F {
    fn into(self) -> NbtTag {
        NbtTag::List(vec![NbtTag::Float(self.0), NbtTag::Float(self.1)])
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Either<F, S> {
    First(F),
    Second(S)
}

impl<F, S> Into<NbtTag> for Either<F, S>
where F: Into<NbtTag>, S: Into<NbtTag> {
    fn into(self) -> NbtTag {
        match self {
            Either::First(first) => first.into(),
            Either::Second(second) => second.into()
        }
    }
}

#[cfg(not(feature = "legacy_uuids"))]
fn uuid_to_int_array(id: Uuid) -> NbtTag {
    let bytes = id.as_bytes().to_vec();
    let first = bytes[0..3].as_ref().read_i32::<BigEndian>().unwrap();
    let second = bytes[4..7].as_ref().read_i32::<BigEndian>().unwrap();
    let third = bytes[8..11].as_ref().read_i32::<BigEndian>().unwrap();
    let fourth = bytes[12..15].as_ref().read_i32::<BigEndian>().unwrap();
    NbtTag::IntArray(vec![first, second, third, fourth])
}

impl Into<NbtTag> for Uuid {
    fn into(self) -> NbtTag {
        #[cfg(feature = "legacy_uuids")]
        return NbtTag::String(self.to_string());
        #[cfg(not(feature = "legacy_uuids"))]
        return uuid_to_int_array(self)
    }
}

#[derive(Debug, Clone)]
pub struct Positive<I> {
    value: I
}

impl<I> Into<NbtTag> for Positive<I> where I: Into<NbtTag> {
    fn into(self) -> NbtTag {
        self.value.into()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3I(pub i32, pub i32, pub i32);

impl Into<NbtTag> for Vec3I {
    fn into(self) -> NbtTag {
        NbtTag::IntArray(vec![self.0, self.1, self.2])
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum GeneralColor {
    White,
    Orange,
    Magenta,
    LightBlue,
    Yellow,
    Lime,
    Pink,
    Gray,
    LightGray,
    Cyan,
    Purple,
    Blue,
    Brown,
    Green,
    Red,
    Black
}

impl Into<NbtTag> for GeneralColor {
    fn into(self) -> NbtTag {
        NbtTag::Byte(self as i8)
    }
}

macro_rules! __positive_impl {
    ($($int:ident),* $(,)*) => {
        $(
        impl Positive<$int> {
            pub fn new(i: $int) -> Self {
                assert!(i >= 0, "Only positive numbers are accepted!");
                Self {
                    value: i
                }
            }
        }

        impl Into<Positive<$int>> for $int {
            fn into(self) -> Positive<$int> {
                Positive::new(self)
            }
        }
        )*
    };
}

__positive_impl!(i32);

#[doc(hidden)]
#[macro_export]
macro_rules! __meta_struct {
    (
        $(
            $name:ident {
                $(
                $stored_name:ident by $field_name:ident: $typ:ident $(<$generic:ident>)?
                ),* $(,)*
            }
        );* $(;)*
    ) => {
        $(
            #[derive(Debug, Clone)]
            pub struct $name {
                $(
                $field_name:Option<$typ $(<$generic>)?>,
                )*
            }

            impl $name {
                pub fn new() -> Self {
                    $name {
                        $(
                        $field_name: None,
                        )*
                    }
                }

                $(
                    pub fn $field_name(&mut self, value: $typ $(<$generic>)*) -> Self {
                        self.$field_name = Some(value);
                        self.clone()
                    }
                )*
            }

            impl MetaContainer for $name {
                fn write_meta<W>(&mut self, writer: &mut W) -> anyhow::Result<()> where W: NbtWriter {
                    let comp = self.tag();
                    writer.write_tag(None, comp)?;
                    Ok(())
                }

                fn tag(&self) -> $crate::nbt::NbtTag {
                    $(
                    let $field_name: NbtTag = Clone::clone(&self.$field_name).into();
                    )*
                    let comp = $crate::nbt::NbtTag::Compound($crate::nbt! {
                        $(
                        $stored_name: $field_name,
                        )*
                    });
                    comp
                }
            }

            impl Into<$crate::nbt::NbtTag> for $name {
                fn into(self) -> $crate::nbt::NbtTag {
                    self.tag()
                }
            }
        )*
    }
}