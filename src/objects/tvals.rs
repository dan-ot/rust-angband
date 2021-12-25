/// A T-Val is an object's most basic type. Used for sorting and categorizing the objects
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd)]
pub enum TVals {
    Null = 0,
    Chest,
    Shot,
    Arrow,
    Bolt,
    Bow,
    Digging,
    Hafted,
    Polearm,
    Sword,
    Boots,
    Gloves,
    Helm,
    Crown,
    Shield,
    Cloak,
    SoftArmor,
    HardArmor,
    DragArmor,
    Light,
    Amulet,
    Ring,
    Staff,
    Wand,
    Rod,
    Scroll,
    Potion,
    Flask,
    Food,
    Mushroom,
    MagicBook,
    PrayerBook,
    NatureBook,
    ShadowBook,
    OtherBook,
    Gold,
    Max
}

pub const SVal_Unknown: i32 = 0;

impl TVals {
    pub fn name(&self) -> &str {
        match self {
            TVals::Null => "none",
            TVals::Chest => "chest",
            TVals::Shot => "shot",
            TVals::Arrow => "arrow",
            TVals::Bolt => "bolt",
            TVals::Bow => "bow",
            TVals::Digging => "digger",
            TVals::Hafted => "hafted",
            TVals::Polearm => "polearm",
            TVals::Sword => "sword",
            TVals::Boots => "boots",
            TVals::Gloves => "gloves",
            TVals::Helm => "helm",
            TVals::Crown => "crown",
            TVals::Shield => "shield",
            TVals::Cloak => "cloak",
            TVals::SoftArmor => "soft armor",
            TVals::HardArmor => "hard armor",
            TVals::DragArmor => "dragon armor",
            TVals::Light => "light",
            TVals::Amulet => "amulet",
            TVals::Ring => "ring",
            TVals::Staff => "staff",
            TVals::Wand => "wand",
            TVals::Rod => "rod",
            TVals::Scroll => "scroll",
            TVals::Potion => "potion",
            TVals::Flask => "flask",
            TVals::Food => "food",
            TVals::Mushroom => "mushroom",
            TVals::MagicBook => "magic book",
            TVals::PrayerBook => "prayer book",
            TVals::NatureBook => "nature book",
            TVals::ShadowBook => "shadow book",
            TVals::OtherBook => "other book",
            TVals::Gold => "gold",
            TVals::Max => ""
        }
    }

    pub fn min_randarts(&self) -> i32 {
        match self {
            TVals::Bow => 4,
            TVals::Hafted => 5,
            TVals::Polearm => 5,
            TVals::Sword => 5,
            TVals::Boots => 4,
            TVals::Gloves => 4,
            TVals::Helm => 3,
            TVals::Crown => 1,
            TVals::Shield => 4,
            TVals::Cloak => 4,
            TVals::SoftArmor => 2,
            TVals::HardArmor => 2,
            TVals::DragArmor => 1,
            TVals::Light => 3,
            TVals::Amulet => 3,
            TVals::Ring => 3,
            _ => 0
        }
    }
}