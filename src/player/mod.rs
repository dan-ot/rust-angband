use crate::bitflags::Bitflag;
use crate::cave::Chunk;
use crate::monsters::Monster;
use crate::monsters::MonsterRace;
use crate::objects::mods::ObjMods;
use crate::objects::tvals::TVals;
use crate::objects::{Effect, ElementInfo, Elements, Object, ObjectKind};
use crate::player::stats::Stats;
use crate::random::Random;
use crate::types::Loc;
use std::collections::HashMap;

pub mod calcs;
pub mod history;
pub mod options;
pub mod stats;

pub enum PlayerFlags {
    None = 0,
    FastShot,
    Bravery30,
    BlessWeapon,
    ZeroFail,
    Beam,
    ChooseSpells,
    KnowMushroom,
    KnowZapper,
    SeeOre,
    NoMana,
    Charm,
    Unlight,
    Rock,
    Steal,
    ShieldBash,
    Evil,
    CombatRegen,
}

pub const STAT_RANGE: usize = 38;
pub const MAX_EXP: i64 = 99_999_999;
pub const KNOW_LEVEL: usize = 30;
pub const MAX_LEVEL: usize = 50;

pub enum SpellFlags {
    SpellLearned,
    SpellWorked,
    SpellForgotten,
}

pub const BTH_PLUS_ADJ: usize = 3;

pub enum Cheater {
    Wizard,
    Debug,
    Jumping,
}

pub enum Digging {
    Rubble = 0,
    Magma,
    Quartz,
    Granite,
    Doors,
}

pub enum Skills {
    DisarmPhys,
    DisarmMagic,
    Device,
    Save,
    Search,
    Stealth,
    ToHitMelee,
    ToHitBow,
    ToHitThrow,
    Digging,
}

// TODO: This is also a linked list in the source
pub struct Quest {
    pub index: i8,
    pub name: String,
    /// Dungeon level
    pub level: i8,
    pub race: MonsterRace,
}

pub struct EquipSlot {
    // TODO: This is probably an enum
    pub eq_type: u16,
    pub name: String,
    pub obj: Object,
}

// TODO: This is also a linked list in the source
pub struct PlayerBody {
    pub name: String,
    pub slots: Vec<EquipSlot>,
}

pub struct PlayerRace {
    pub name: String,
    pub ridx: usize,

    /// Hit-dice modifier
    pub r_mhp: i32,
    /// Experience factor
    pub r_exp: i32,

    /// Base age
    pub b_age: i32,
    /// Mod age
    pub m_age: i32,

    pub base_hgt: i32,
    pub mod_hgt: i32,
    pub base_wgt: i32,
    pub mod_wgt: i32,

    pub infra: i32,

    /// Race body
    pub body: i32,

    pub r_adj: HashMap<Stats, i32>,
    pub r_skills: HashMap<Skills, i32>,

    /// Object flags
    pub flags: Bitflag,
    /// Player Flags
    pub pflags: Bitflag,

    pub history: Vec<HistoryChart>,
    pub el_info: HashMap<Elements, ElementInfo>,
}

pub struct PlayerShape {
    pub name: String,

    pub sidx: usize,

    pub to_a: i32,
    pub to_h: i32,
    pub to_d: i32,

    pub skills: HashMap<Skills, i32>,
    /// ObjectFlags
    pub flags: Bitflag,
    /// PlayerFlags
    pub pflags: Bitflag,

    pub modifiers: HashMap<ObjMods, i32>,
    pub el_info: HashMap<Elements, ElementInfo>,

    pub effects: Vec<Effect>,

    // Each player_blow from the source is just a name and a pointer to the next.
    // A Vec<> also lets us fetch the length/count of the set, replacing num_blows.
    pub blows: Vec<String>,
}

pub struct StartItem {
    pub tval: TVals,
    // TODO: this is an enum of some kind, or an enum of enums
    pub sval: i32,
    pub min: i32,
    pub max: i32,
    /// Exclusion indices - which birth options will exclude this item...
    pub eopts: Vec<i32>,
}

pub struct MagicRealm {
    pub code: String,
    pub name: String,
    pub stat: Stats,
    pub verb: String,
    pub spell_noun: String,
    pub book_noun: String,
}

pub struct ClassSpell {
    pub name: String,
    pub text: String,

    // TODO: is this a set?
    pub effect: Effect,

    // TODO: This is duplicated by the class book which contains this spell...
    pub realm: MagicRealm,

    /// The index of this spell for this class
    pub sidx: usize,
    /// The index into the player's books array
    pub bidx: usize,
    pub slevel: i32,
    pub smana: i32,
    pub sfail: i32,
    pub sexp: i32,
}

pub struct ClassBook {
    pub tval: TVals,
    // TODO: this is an enum of some kind, or an enum of enums
    pub sval: i32,
    pub dungeon: bool,
    pub realm: MagicRealm,
    pub spells: Vec<ClassSpell>,
}

pub struct ClassMagic {
    pub spell_first: i32,
    pub spell_weight: i32,
    pub books: Vec<ClassBook>,
    // TODO: This could be derived by a flatmap of the books...
    pub total_spells: i32,
}

pub struct PlayerClass {
    pub name: String,
    pub cidx: usize,
    pub title: Vec<String>,

    pub c_adj: HashMap<Stats, i32>,
    pub c_skills: HashMap<Skills, i32>,
    pub x_skills: HashMap<Skills, i32>,

    pub c_mhp: i32,
    pub c_exp: i32,

    /// ObjectFlags
    pub flags: Bitflag,
    /// PlayerFlags
    pub pflags: Bitflag,

    pub max_attacks: i32,
    pub min_weight: i32,
    pub att_multiply: i32,

    pub start_items: Vec<StartItem>,
    pub magic: Option<ClassMagic>,
}

pub struct PlayerAbility {
    /// ObjectFlage or PlayerFlag or Element index? Is this an enum/union?
    pub index: u16,
    pub ab_type: String,
    pub name: String,
    pub desc: String,
    pub group: i32,
    pub value: i32,
}

pub struct HistoryEntry {
    pub succ: HistoryChart,
    pub isucc: i32,
    pub roll: i32,
    pub text: String,
}

/// Histories are a graph of History Charts; each chart contains a set of
/// individual entries for that chart, and each entyr contains a text description
/// and a successor chart to move history generation to.
///
/// History generation works by walking the graph from the starting chart
/// for each race, picking a random entry (with weighted probability) each time.
pub struct HistoryChart {
    pub entries: Vec<HistoryEntry>,
    pub idx: u16,
}

pub struct PlayerHistory {
    pub entries: Vec<history::HistoryInfo>,
}

/// All the variable state that changes when you put on/take off equipment.
/// Player flags are not currently variable, but useful here so monsters can
/// learn them.
pub struct PlayerState {
    /// Equipment stat bonuses
    pub stat_add: HashMap<Stats, i32>,
    /// Indices into stat tables
    pub stat_ind: HashMap<Stats, usize>,
    /// Current modified stats
    pub stat_use: HashMap<Stats, i32>,
    /// Maximal modified stats
    pub stat_top: HashMap<Stats, i32>,

    pub skills: HashMap<Skills, i32>,

    pub speed: i32,

    /// Number of blows in hundredths
    pub num_blows: i32,
    /// Number of shots tenths
    pub hum_shots: i32,
    pub num_moves: i32,

    pub ammo_mult: i32,
    pub ammo_tval: TVals,

    pub ac: i32,
    pub dam_red: i32,
    pub perc_dam_red: i32,
    pub to_a: i32,
    pub to_h: i32,
    pub to_d: i32,

    /// Infravision range
    pub see_infra: i32,

    /// Light radius (if any)
    pub cur_light: i32,

    /// Currently weilding a heavy weapon
    pub heav_wield: bool,
    /// Currently shooting a heavy shooter
    pub heavy_shoot: bool,
    /// Blessed (or blunt) weapon - for Priest?
    pub bless_wield: bool,
    /// Mana-draining armor - for Wizard?
    pub cumber_armor: bool,

    /// ObjectFlags
    pub flags: Bitflag,
    /// PlayerFlags
    pub pflags: Bitflag,
    pub el_info: HashMap<Elements, ElementInfo>,
}

/// Temporary, derived, player-related variables used during play but not saved
pub struct PlayerUpkeep {
    pub playing: bool,
    pub autosave: bool,
    pub generate_level: bool,
    pub only_partial: bool,
    pub dropping: bool,

    pub energy_use: i32,
    pub new_spells: i32,

    // TODO: these are pointers in the source. We are not meant to have ownership here.
    // Also, I think this is a monster/object union - not sure if you can track both
    pub health_who: Monster,
    pub monster_race: MonsterRace,
    pub object: Object,
    pub object_kind: ObjectKind,

    /// PlayerNotice Bitflags for pending actions such as reordering inventory, ignoring, etc.
    pub notice: Bitflag,
    /// PlayerUpdate Bitflags for recalculations needed such as HP, or visible area
    pub update: Bitflag,
    /// PlayerRedraw Bitflags for things that have changed, and just need to be redrawn by the UI, such as HP, Speed, etc.
    pub redraw: Bitflag,

    /// Used by the UI to decide whether to start off showing equipment or inventory listings
    /// when offering a choice. Maybe a bool?
    pub command_wrk: i32,

    pub create_up_stair: bool,
    pub create_down_stair: bool,
    pub light_level: bool,
    pub arena_level: bool,

    pub resting: i32,

    pub running: i32,
    pub running_withpathfind: bool,
    pub running_firststep: bool,

    // TODO: These are ** pointer-pointers in the source. An array of pointers?
    pub quiver: Vec<Object>,
    pub inven: Vec<Object>,
    pub total_weight: i32,
    pub inven_cnt: i32,
    pub equip_cnt: i32,
    pub quiver_cnt: i32,
    pub recharge_pow: i32,
}

/// Most of the "player" information goes here.
///
/// This structure gives us a large collection of layer variables.
///
/// This entire structure is whiped when a new character is born.
///
/// This structure is more or less laid out so that the information which must be stored
/// in the savefile precedes all the information which can be recomputed as needed.
pub struct Player {
    pub race: PlayerRace,
    pub class: PlayerClass,

    pub grid: Loc,

    pub hitdie: u8,
    pub expfact: u8,

    pub age: i16,
    pub ht: i16,
    pub wt: i16,

    pub au: i32,

    pub max_depth: i16,
    pub recall_depth: i16,
    pub depth: i16,

    pub max_lev: i16,
    pub lev: i16,

    pub max_exp: i32,
    pub exp: i32,
    /// Cur exp frac (times 2^16)
    pub exp_frac: u16,

    pub mhp: i16,
    pub chp: i16,
    /// Cur hit frac (times 2^16)
    pub chp_frac: u16,

    pub stat_max: HashMap<Stats, i16>,
    pub stat_cur: HashMap<Stats, i16>,
    pub stat_map: HashMap<Stats, i16>,

    pub timed: Vec<i16>,

    pub word_recall: i16,
    pub deep_descent: i16,

    pub energy: i16,
    pub total_energy: u32,
    pub resting_turn: u32,

    pub food: i16,

    pub unignoring: u8,

    pub spell_flags: Vec<u8>,
    pub spell_order: Vec<u8>,

    pub full_name: String,
    pub died_from: String,
    pub history: String,
    pub quests: Vec<Quest>,
    pub total_winner: u16,

    /// Cheating flags
    pub noscore: u16,

    pub is_dead: bool,
    pub wizard: bool,

    /// HP gained per level
    pub player_hp: Vec<i16>,

    pub au_birth: i32,
    pub stat_birth: HashMap<Stats, i16>,
    pub ht_birth: i16,
    pub wt_birth: i16,

    pub opts: options::PlayerOptions,
    pub hist: PlayerHistory,

    pub body: PlayerBody,
    pub shape: PlayerShape,

    // TODO: is this a set?
    pub gear: Vec<Object>,
    pub gear_k: Vec<Object>,

    /// Object knowledge ("runes")
    pub obj_k: Vec<Object>,
    pub cave: Chunk,

    pub state: PlayerState,
    pub known_state: PlayerState,
    pub upkeep: PlayerUpkeep,
}

impl Player {
    pub fn stat_inc(&mut self, random: &mut Random, stat: &Stats) -> bool {
        let v = *self
            .stat_cur
            .get(stat)
            .expect("Unrecognized stat for stat increase!"); // Stats are a closed set, so this should never panic.

        if v >= 18 + 100 {
            false
        } else {
            if v < 18 {
                self.stat_cur.entry(*stat).and_modify(|s| *s += 1);
            } else if v < 18 + 90 {
                let mut gain = (((18 + 100) - v) / 2 + 3) / 2;
                if gain < 1 {
                    gain = 1;
                }

                self.stat_cur.entry(*stat).and_modify(|s| {
                    let added = (random.randint1(gain as i32) as i16) + gain / 2;
                    if *s + added > 18 + 99 {
                        *s = 18 + 99
                    } else {
                        *s += added
                    }
                });
            } else {
                self.stat_cur.entry(*stat).and_modify(|s| *s = 18 + 100);
            }

            let nowits = *self.stat_cur.get(stat).unwrap();
            if nowits > *self.stat_max.get(stat).unwrap() {
                self.stat_max.entry(*stat).and_modify(|s| *s = nowits);
            }

            self.upkeep
                .update
                .turn_on(calcs::PlayerUpdate::Bonus as usize);

            true
        }
    }

    pub fn stat_dec(&mut self, stat: &Stats, permanent: &bool) -> bool {
        let mut cur = *self.stat_cur.get(stat).unwrap();
        let mut max = *self.stat_max.get(stat).unwrap();
        let mut res;

        if cur > 18 + 10 {
            cur -= 10;
        } else if cur > 18 {
            cur = 18;
        } else if cur > 3 {
            cur -= 1;
        }

        res = cur != *self.stat_cur.get(stat).unwrap();

        if *permanent {
            if max > 18 + 10 {
                max -= 10;
            } else if max > 18 {
                max = 18;
            } else if max > 3 {
                max -= 1;
            }

            res = max != *self.stat_cur.get(stat).unwrap();
        }

        if res {
            self.stat_cur.entry(*stat).and_modify(|s| *s = cur);
            self.stat_max.entry(*stat).and_modify(|s| *s = max);
            self.upkeep
                .update
                .turn_on(calcs::PlayerUpdate::Bonus as usize);
            self.upkeep
                .redraw
                .turn_on(calcs::PlayerRedraw::Stats as usize);
        }

        res
    }
}

pub const PLAYER_EXP: [i64; 50] = [
    10, 25, 45, 70, 100, 140, 200, 280, 380, 500, 650, 850, 1100, 1400, 1800, 2300, 2900, 3600,
    4400, 5400, 6800, 8400, 10200, 12500, 17500, 25000, 35000, 50000, 75000, 100000, 150000,
    200000, 275000, 350000, 450000, 550000, 700000, 850000, 1000000, 1250000, 1500000, 1800000,
    2100000, 2400000, 2700000, 3000000, 3500000, 4000000, 4500000, 5000000,
];
