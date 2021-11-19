use crate::stats::Stats;
use crate::objects::flags::ObjectFlags;
use crate::bitflags::Bitflag;
use std::collections::HashMap;
use crate::objects::tvals::TVals;

mod tvals;
mod flags;
mod kinds;
mod mods;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum FlagType {
    /// Sustains a stat
    Sust, 
    /// Protection from an effect
    Prot,
    /// Good property, suitable for ego items
    Misc,
    /// Only for light sources
    Light,
    /// Only for melee weapons
    Melee,
    /// Undesirable
    Bad,
    /// Only for diggers
    Dig,
    /// Only for throwables
    Throw
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
/// How object flags are identified
pub enum FlagId {
    /// Normal Id on use
    Normal = 0,
    /// Obvious after time
    Timed,
    /// Obvious after wield
    Wield
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub enum PropertyType {
    None = 0,
    Stat,
    Mod,
    Flag,
    Ignore,
    Resist,
    Vuln,
    Imm
}

#[derive(Debug, Clone)]
pub struct ObjectProperty {
    pub prop_type: PropertyType,
    pub subtype: FlagType,
    pub id_type: FlagId,
    pub power: i32,
    pub index: usize,
    pub mult: i32,
    pub type_mult: HashMap<TVals, i32>,
    pub name: String,
    pub adjective: String,
    pub neg_adj: String,
    pub msg: String,
    pub desc: String
}

#[derive(Debug, Clone)]
pub struct AllProperties {
    pub properties: HashMap<PropertyType, Vec<ObjectProperty>>
}

impl AllProperties {
    fn lookup_obj_property(&self, obj_type: &PropertyType, index: &usize) -> Option<&ObjectProperty> {
        match self.properties.get(obj_type) {
            Some (t) => t.get(*index),
            None => None
        }
    }
}

/// Initialize the given Bitflag with all
pub fn create_obj_flag_mask_by_type(f: &mut Bitflag, props: &AllProperties, args: Box<dyn Iterator<Item = FlagType>>) -> () {
    f.wipe();
    for arg in args {
        for prop in props.properties[&PropertyType::Flag].iter().filter(|p| { p.subtype == arg }) {
            f.turn_on(&prop.index);
        }
    }
}

pub fn create_obj_flag_mask_by_id(f: &mut Bitflag, props: &AllProperties, args: Box<dyn Iterator<Item = FlagId>>) -> () {
    f.wipe();
    for arg in args {
        for prop in props.properties[&PropertyType::Flag].iter().filter(|p| { p.id_type == arg }) {
            f.turn_on(&prop.index);
        }
    }
}

pub fn flag_message(all_props: &AllProperties, flag: &usize, name: &str) -> () {
    let prop = all_props.lookup_obj_property(&PropertyType::Flag, flag);
    match prop {
        Some (p) => {
            let m = &p.msg;
            let message = m.replace("{name}", name);
            // TODO: Need to implement the message handler
        },
        None => ()
    }
}

pub fn sustain_flag(stat: Stats) -> ObjectFlags {
    match stat {
        Stats::Str => ObjectFlags::SustStr,
        Stats::Con => ObjectFlags::SustCon,
        Stats::Dex => ObjectFlags::SustDex,
        Stats::Int => ObjectFlags::SustInt,
        Stats::Wis => ObjectFlags::SustWis
    }
}

/// Base: Acid to Cold. Hight: Poison to Disen.
pub enum Elements {
    Acid = 0,
    Elec,
    Fire,
    Cold,
    Pois,
    Light,
    Dark,
    Sound,
    Shard,
    Nexus,
    Nether,
    Chaos,
    Disen,
    Water,
    Ice,
    Gravity,
    Inertia,
    Force,
    Time,
    Plasma,
    Meteor,
    Missile,
    Mana,
    HolyOrb,
    Arrow
}

pub enum Origin {
    None = 0,
    Floor,
    Chest,
    Special,
    Pit,
    Vault,
    Labyrinth,
    Cavern,
    Rubble,
    Mixed,
    Drop,
    DropSpecial,
    DropPit,
    DropVault,
    Stats,
    Acquire,
    Store,
    Stolen,
    Birth,
    Cheat,
    DropBreed,
    DropSummon,
    DropUnknown,
    DropPoly,
    DropMimic,
    DropWizard
}

impl Origin {
    pub fn describe(&self, subject: &str) -> String {
        match self {
            Origin::None | Origin::Mixed | Origin::Stats | Origin::Stolen => String::from(""),
            Origin::Drop | Origin::DropSpecial | Origin::DropPit | Origin::DropVault
                | Origin::DropBreed | Origin::DropSummon | Origin::DropPoly
                | Origin::DropMimic | Origin::DropWizard => "Dropped by ".to_owned() + subject,
            Origin::DropUnknown => "Dropped by an unknown monster ".to_owned() + subject,
            Origin::Floor => "Found lying on the floor ".to_owned() + subject,
            Origin::Chest => "Taken from a chest found ".to_owned() + subject,
            Origin::Special => "Found lying on the floor of a special room ".to_owned() + subject,
            Origin::Pit => "Found lying on the floor of a pit ".to_owned() + subject,
            Origin::Vault => "Found lying on the floor in a vault ".to_owned() + subject,
            Origin::Labyrinth => "Found lying on the floor of a labyrinth ".to_owned() + subject,
            Origin::Cavern => "Found lying on the floor of a cavern ".to_owned() + subject,
            Origin::Rubble => "Found under some rubble ".to_owned() + subject,
            Origin::Acquire => "Conjured forth by magic ".to_owned() + subject,
            Origin::Store => "Bought from a store".to_owned(),
            Origin::Birth => "An inheritance from your family ".to_owned(),
            Origin::Cheat => "Created by debug option".to_owned(),
        }
    }
}

/// An inventory object. The original serves a lot of purposes - they describe the prototypes
/// ala Flyweight pattern, and they also store navigation properties to support doubly-linked lists.
/// It will be difficult to tease all that out properly.
#[derive(Debug, Clone)]
pub struct Object {

}