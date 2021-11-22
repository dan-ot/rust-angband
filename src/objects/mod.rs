use crate::bitflags::Bitflag;
use crate::dice::Dice;
use crate::monsters::MonsterRace;
use crate::objects::flags::ObjectFlags;
use crate::objects::mods::ObjMods;
use crate::objects::tvals::TVals;
use crate::random::Diceroll;
use crate::stats::Stats;
use crate::types::Loc;
use std::collections::HashMap;

mod flags;
mod kinds;
mod mods;
mod tvals;

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
    Throw,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
/// How object flags are identified
pub enum FlagId {
    /// Normal Id on use
    Normal = 0,
    /// Obvious after time
    Timed,
    /// Obvious after wield
    Wield,
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
    Imm,
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
    pub desc: String,
}

#[derive(Debug, Clone)]
pub struct AllProperties {
    pub properties: HashMap<PropertyType, Vec<ObjectProperty>>,
}

impl AllProperties {
    fn lookup_obj_property(
        &self,
        obj_type: &PropertyType,
        index: &usize,
    ) -> Option<&ObjectProperty> {
        match self.properties.get(obj_type) {
            Some(t) => t.get(*index),
            None => None,
        }
    }
}

/// Initialize the given Bitflag with all
pub fn create_obj_flag_mask_by_type(
    f: &mut Bitflag,
    props: &AllProperties,
    args: Box<dyn Iterator<Item = FlagType>>,
) -> () {
    f.wipe();
    for arg in args {
        for prop in props.properties[&PropertyType::Flag]
            .iter()
            .filter(|p| p.subtype == arg)
        {
            f.turn_on(&prop.index);
        }
    }
}

pub fn create_obj_flag_mask_by_id(
    f: &mut Bitflag,
    props: &AllProperties,
    args: Box<dyn Iterator<Item = FlagId>>,
) -> () {
    f.wipe();
    for arg in args {
        for prop in props.properties[&PropertyType::Flag]
            .iter()
            .filter(|p| p.id_type == arg)
        {
            f.turn_on(&prop.index);
        }
    }
}

pub fn flag_message(all_props: &AllProperties, flag: &usize, name: &str) -> () {
    let prop = all_props.lookup_obj_property(&PropertyType::Flag, flag);
    match prop {
        Some(p) => {
            let m = &p.msg;
            let message = m.replace("{name}", name);
            // TODO: Need to implement the message handler
        }
        None => (),
    }
}

pub fn sustain_flag(stat: Stats) -> ObjectFlags {
    match stat {
        Stats::Str => ObjectFlags::SustStr,
        Stats::Con => ObjectFlags::SustCon,
        Stats::Dex => ObjectFlags::SustDex,
        Stats::Int => ObjectFlags::SustInt,
        Stats::Wis => ObjectFlags::SustWis,
    }
}

/// Base: Acid to Cold. Hight: Poison to Disen.
#[derive(Debug, Clone)]
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
    Arrow,
}

#[derive(Debug, Clone)]
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
    DropWizard,
}

impl Origin {
    pub fn describe(&self, subject: &str) -> String {
        match self {
            Origin::None | Origin::Mixed | Origin::Stats | Origin::Stolen => String::from(""),
            Origin::Drop
            | Origin::DropSpecial
            | Origin::DropPit
            | Origin::DropVault
            | Origin::DropBreed
            | Origin::DropSummon
            | Origin::DropPoly
            | Origin::DropMimic
            | Origin::DropWizard => "Dropped by ".to_owned() + subject,
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

// TODO: This is an enum/union between projections, timed effects, and radius effects...will have to suss out later
// TODO: In the source, each Effect is by default part of a linked list of effects. I'll have to watch out for usage, whether some places are using it as part of a list or as a single thing
#[derive(Debug, Clone)]
pub struct Effect {
    // Next -> Effects are by default part of a list
    // pub index: i32 -> This probably refers to the index of the effect in that list. If not, add this back in.
    /// Dice expression used in the effect
    pub dice: Dice,
    /// Y coordinate or distance
    pub y: i32,
    /// X coordinate or distance
    pub x: i32,
    /// Projection type, Timed effec type, etc. <- comment from source shows we should look at using an enum instead of a shared-space struct
    pub subtype: i32,
    /// Radius of the effect (if it has one)
    pub radius: i32,
    /// Extra parameter to be passed to the handler
    pub other: i32,
    /// Message for deth or whatever
    pub msg: String,
}

// TODO: In the source, this is also a linked list node
pub struct ChestTrap {
    pub name: String,
    pub code: String,
    pub level: i32,
    /// TODO: Is this a single effect, or a list of them?
    pub effect: Effect,
    pub pval: i32,
    pub destroy: bool,
    pub magic: bool,
    pub msg: String,
    pub msg_death: String,
}

// TODO: In the source, this is also a linked list node
pub struct Brand {
    pub code: String,
    pub name: String,
    pub verb: String,
    pub resist_flag: i32,
    pub vuln_flag: i32,
    pub multiplier: i32,
    pub o_multiplier: i32,
    pub power: i32,
}

// TODO: In the source, this is also a linked list node
// This is the same data schema as a Brand, what's the difference?
pub struct Slay {
    pub code: String,
    pub name: String,
    pub verb: String,
    pub resist_flag: i32,
    pub vuln_flag: i32,
    pub multiplier: i32,
    pub o_multiplier: i32,
    pub power: i32,
}

// TODO: In the source, this is also a linked list
pub struct Curse {
    pub name: String,
    /// This was *bool - is that an array?
    pub poss: bool,
    /// This was *object - is that an array?
    pub obj: Object,
    pub conflict: String,
    pub conflict_flags: Bitflag,
    pub desc: String,
}

pub enum ElInfo {
    Hates,
    Ignore,
    Random,
}

#[derive(Debug, Clone)]
pub struct ElementInfo {
    pub res_level: i16,
    pub flags: Bitflag,
}

// TODO: In the source, this is also a linked list
#[derive(Debug, Clone)]
pub struct Activation {
    pub name: String,
    pub index: i32,
    pub aim: bool,
    pub power: i32,
    /// TODO: Is this a linked list?
    pub effect: Effect,
    pub message: String,
    pub desc: String,
}

pub struct ActivationService {
    pub activations: Vec<Activation>,
}

// TODO: In the source, this is also a linked list
#[derive(Debug, Clone)]
pub struct ObjectBase {
    pub name: String,
    pub tval: TVals,
    pub attr: i32,
    /// ObjectFlags
    pub flags: Bitflag,
    /// KindFlags
    pub kind_flags: Bitflag,
    pub el_info: HashMap<Elements, ElementInfo>,
    pub break_perc: i32,
    pub max_stack: i32,
    pub num_svals: i32,
}

pub struct ObjectBaseService {
    pub kb_info: Vec<ObjectBase>,
}

/// Information about object kinds, including player knowledge
/// TODO: Separate user state from basic object information. A good clue is the ArtifactUpkeep struct.
/// TODO: In the source, this is also a linked list
#[derive(Debug, Clone)]
pub struct ObjectKind {
    pub name: String,
    pub text: String,

    /// This was *object_base - is that an array?
    pub base: ObjectBase,
    pub index: u32,
    pub tval: TVals,
    /// Object sub-type. Is this actually an enum? Or a union of enums?
    pub sval: i32,

    /// Extra parameter
    pub pval: Diceroll,

    /// Bonus to hit
    pub to_h: Diceroll,
    /// Bonus to damage
    pub to_d: Diceroll,
    /// Bonus to Armor
    pub to_a: Diceroll,
    /// Base Armor
    pub ac: i32,

    /// Damage Dice
    pub dd: i32,
    /// Damage Sides
    pub ds: i32,
    /// Weight, in 1/10 lbs
    pub weight: i32,

    /// Base cost
    pub cost: i32,

    /// ObjectFlags
    pub flags: Bitflag,
    /// KindFlags
    pub kind_flags: Bitflag,

    pub modifiers: HashMap<ObjMods, Diceroll>,
    pub el_info: HashMap<Elements, ElementInfo>,

    pub brands: Vec<bool>,
    pub slays: Vec<bool>,
    /// Curse powers
    pub curses: Vec<i32>,

    /// Default attribute
    pub d_attr: i8,
    /// Default character
    pub d_char: char,

    /// Allocation: commonness
    pub alloc_prob: i32,
    /// Highest normal dungeon level
    pub alloc_min: i32,
    /// Lowest normal dungeon level
    pub alloc_max: i32,
    /// Level (difficulty of activation)
    pub level: i32,

    /// Actifact-like activation
    pub activation: Activation,
    /// Effect this item produces (is this a linked list?)
    pub effect: Effect,
    /// Power of the item's effect
    pub power: i32,
    pub effect_msg: String,
    pub vis_msg: String,

    // TODO: This pair looks like an enum/DU
    /// Recharge time (rods/activation)
    pub time: Diceroll,
    /// Number of charges (staves/wands)
    pub charge: Diceroll,

    /// Probability of generating more than one
    pub gen_mult_prob: i32,
    /// Number to generate
    pub stack_size: Diceroll,

    /// Special object flavor. In the source, this was a pointer which could be set to 0 (NULL)
    pub flavor: Option<Flavor>,

    // Also saved in savefile
    /// Autoinscription quark
    pub note_aware: String,
    /// Autoinscription quark
    pub note_unaware: String,

    /// Set if player is aware of the kind's effects
    pub aware: bool,
    /// Set if kind has been tried
    pub tried: bool,

    /// Ignore settings
    pub ignore: u8,
    /// Kind has been seen (to despoilify ignore menus)
    pub everseen: bool,
}

/// Available in the k_info, unknown_item_kind, unknown_gold_kind, pile_kind, and curse_object_kind sets
pub struct ObjectKindService {
    pub k_info: Vec<ObjectKind>,
    pub unknown_item_kind: Vec<ObjectKind>,
    pub unknown_gold_kind: Vec<ObjectKind>,
    pub pile_kind: Vec<ObjectKind>,
    pub curse_object_kind: Vec<ObjectKind>,
}

/// Unchanging information about artifacts.
/// TODO: In this source, this is also a linked list.
#[derive(Debug, Clone)]
pub struct Artifact {
    pub name: String,
    pub text: String,

    /// Artifact Index (for cross-referencing saved state...)
    pub aidx: u32,

    pub tval: TVals,
    /// Artifact sub-type. Probably an enum, or an enum/union of enums
    pub sval: i32,

    pub to_h: i32,
    pub to_d: i32,
    pub to_a: i32,
    pub ac: i32,

    pub dd: i32,
    pub ds: i32,
    pub weight: i32,

    pub cost: i32,

    /// ObjectFlags
    pub flags: Bitflag,

    pub modifiers: HashMap<ObjMods, i32>,
    pub el_info: HashMap<Elements, ElementInfo>,

    pub brands: Vec<bool>,
    pub slays: Vec<bool>,
    pub curses: Vec<i32>,

    pub level: i32,

    pub alloc_prob: i32,
    pub alloc_min: i32,
    pub alloc_max: i32,

    pub activation: Activation,
    pub alt_msg: String,

    pub time: Diceroll,
}

/// Information about artifacts that changes during the course of play;
/// except for aidx, saved to the save file
pub struct ArtifactUpkeep {
    pub aidx: u32,
    pub created: bool,
    pub seen: bool,
    pub everseen: bool,
}

pub struct ArtifactService {
    pub a_info: Vec<Artifact>,
    pub aup_info: Vec<ArtifactUpkeep>,
}

/// TODO: In the source, this is also a linked list
#[derive(Debug, Clone)]
pub struct EgoItem {
    pub name: String,
    pub text: String,

    /// Ego Item Index
    pub eidx: u32,

    /// Ego item "cost"
    pub cost: i32,

    /// ObjectFlags
    pub flags: Bitflag,
    /// ObjectFlags to remove
    pub kinds_off: Bitflag,
    /// KindFlags
    pub kind_flags: Bitflag,

    pub modifiers: HashMap<ObjMods, Diceroll>,
    pub min_modifiers: HashMap<ObjMods, i32>,
    pub el_info: HashMap<Elements, ElementInfo>,

    pub brands: Vec<bool>,
    pub slays: Vec<bool>,
    pub curses: Vec<i32>,

    pub rating: i32,
    pub alloc_prob: i32,
    pub alloc_min: i32,
    pub alloc_max: i32,

    /// Replaces a linked list of kind indices
    pub poss_items: Vec<u32>,

    pub to_h: Diceroll,
    pub to_d: Diceroll,
    pub to_a: Diceroll,

    pub min_to_h: i32,
    pub min_to_d: i32,
    pub min_to_a: i32,

    /// TODO: Is this a vector?
    pub effect: Effect,
    pub effect_msg: String,
    pub time: Diceroll,

    pub everseen: bool,
}

pub struct EgoItemService {
    pub e_info: Vec<EgoItem>,
}

pub enum ObjectNotice {
    Worn,
    Assessed,
    Ignore,
    Imagined,
}

impl ObjectNotice {
    pub fn max() -> usize {
        4
    }
}

#[derive(Debug, Clone)]
pub struct CurseData {
    pub power: i32,
    pub timeout: i32,
}

/// An inventory object. The original serves a lot of purposes - they describe the prototypes
/// ala Flyweight pattern, and they also store navigation properties to support doubly-linked lists.
/// It will be difficult to tease all that out properly.
///
/// held_m_idx is used to represent which monster is holding the stack of objects which this object
/// is a part of. Objects being held have (0, 0) as their grid.
#[derive(Debug, Clone)]
pub struct Object {
    pub kind: Option<ObjectKind>,
    pub ego: Option<EgoItem>,
    pub artifact: Option<Artifact>,

    // TODO: This is a pointer in the source, what's the purpose?
    // pub known: Object,
    pub oidx: u16,

    pub grid: Loc,

    pub tval: TVals,
    pub sval: u8,

    pub pval: i16,
    pub weight: i16,

    pub dd: i8,
    pub ds: i8,
    pub ac: i16,
    pub to_a: i16,
    pub to_h: i16,
    pub to_d: i16,

    /// ObjectFlags
    pub flags: Bitflag,
    pub modifiers: HashMap<ObjMods, i16>,
    pub el_info: HashMap<Elements, ElementInfo>,

    // TODO: Are these parallel arrays mapping global list members to local state?
    pub brands: Vec<bool>,
    pub slays: Vec<bool>,
    pub curses: Vec<CurseData>,

    // TODO: Is this a set?
    pub effect: Option<Effect>,
    pub effect_msg: String,
    pub activation: Option<Activation>,
    pub time: Diceroll,
    pub timeout: i16,

    /// Number of items (quantity?)
    pub number: i8,
    /// ObjectNotice
    pub notice: Bitflag,

    pub held_m_idx: Option<i16>,
    pub mimicking_m_idx: Option<i16>,

    pub origin: Origin,
    pub origin_depth: i8,
    pub origin_race: Option<MonsterRace>,

    pub note: String,
}

impl Object {
    pub fn default() -> Object {
        Object {
            kind: None,
            ego: None,
            artifact: None,
            oidx: 0,
            grid: Loc::zero(),
            tval: TVals::Null,
            sval: 0,
            pval: 0,
            weight: 0,
            dd: 0,
            ds: 0,
            ac: 0,
            to_a: 0,
            to_h: 0,
            to_d: 0,
            flags: Bitflag::new(ObjectFlags::max()),
            modifiers: HashMap::<ObjMods, i16>::new(),
            el_info: HashMap::<Elements, ElementInfo>::new(),
            brands: vec![],
            slays: vec![],
            curses: vec![],
            effect: None,
            effect_msg: String::from(""),
            activation: None,
            time: Diceroll::new(0, 0, 0, 0),
            timeout: 0,
            number: 0,
            notice: Bitflag::new(ObjectNotice::max()),
            held_m_idx: None,
            mimicking_m_idx: None,
            origin: Origin::None,
            origin_depth: 0,
            origin_race: None,
            note: String::from("")
        }
    }
}

// TODO: In the source, this is a linked list
#[derive(Debug, Clone)]
pub struct Flavor {
    pub text: String,
    pub tval: TVals,
    pub sval: u8,
    pub d_attr: u8,
    pub d_char: char
}

pub struct FlavorService {
    pub flavors: Vec<Flavor>
}
