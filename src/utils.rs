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
