use crate::events::{MessageQueue, GameEvent, GameEventData, Message as EvMessage};
use crate::colors::Colors;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum MessageType {
    Generic,
    Birth,
    Hit,
    Miss,
    Flee,
    Drop,
    Kill,
    Level,
    Death,
    Study,
    Teleport,
    Shoot,
    Quaff,
    ZapRod,
    Walk,
    TPOther,
    HitWall,
    Eat,
    Store1,
    Store2,
    Store3,
    Store4,
    Dig,
    OpenDoor,
    ShutDoor,
    TPLevel,
    Bell,
    NothingToOpen,
    LockpickFail,
    StairsDown,
    HitpointWarn,
    ActArtifact,
    UseStaff,
    Destroy,
    MonHit,
    MonTouch,
    MonPunch,
    MonKick,
    MonClaw,
    MonBite,
    MonSting,
    MonButt,
    MonCrush,
    MonEngulf,
    MonCrawl,
    MonDrool,
    MonSpit,
    MonGaze,
    MonWail,
    MonSpore,
    MonBeg,
    MonInsult,
    MonMoan,
    Recover,
    Blind,
    Confused,
    Poisoned,
    Afraid,
    Paralyzed,
    Drugged,
    Speed,
    Slow,
    Shield,
    Blessed,
    Hero,
    Berserk,
    Bold,
    ProtEvil,
    Invuln,
    SeeInvis,
    Infrared,
    ResAcid,
    ResElec,
    ResFire,
    ResCold,
    ResPois,
    Stun,
    Cut,
    StairsUp,
    StoreEnter,
    StoreLeave,
    StoreHome,
    Money1,
    Money2,
    Money3,
    ShootHit,
    Stores,
    Lockpick,
    Disarm,
    IdentBad,
    IdentEgo,
    IdentArt,
    BrElements,
    BrFrost,
    BrElec,
    BrAcid,
    BrGas,
    BrFire,
    BrDisen,
    BrChaos,
    BrShards,
    BrSound,
    BrLight,
    BrDark,
    BrNether,
    BrNexus,
    BrTime,
    BrInertia,
    BrGravity,
    BrPlasma,
    BrForce,
    SumMonster,
    SumAinu,
    SumUndead,
    SumAnimal,
    SumSpider,
    SumHound,
    SumHydra,
    SumDemon,
    SumDragon,
    SumHiUndead,
    SumHiDragon,
    SumHiDemon,
    SumWraith,
    SumUnique,
    Wield,
    Quiver,
    Cursed,
    Rune,
    Hungry,
    Notice,
    AmbientDay,
    AmbientNite,
    AmbientDng1,
    AmbientDng2,
    AmbientDng3,
    AmbientDng4,
    AmbientDng5,
    CreateTrap,
    Shriek,
    CastFear,
    HitGood,
    HitGreat,
    HitSuperb,
    HitHiGreat,
    HitHiSuperb,
    Spell,
    Prayer,
    KillUnique,
    KillKing,
    DrainStat,
    Multiply,
    Scramble
}

impl MessageType {
    pub fn lookup_by_name_or_sound(name: &str) -> Option<MessageType> {
        match name {
            "generic" => Some (MessageType::Generic),
            "birth" => Some (MessageType::Birth),
            "hit" => Some (MessageType::Hit),
            "miss" => Some (MessageType::Miss),
            "flee" => Some (MessageType::Flee),
            "drop" => Some (MessageType::Drop),
            "kill" => Some (MessageType::Kill),
            "level" => Some (MessageType::Level),
            "death" => Some (MessageType::Death),
            "study" => Some (MessageType::Study),
            "teleport" => Some (MessageType::Teleport),
            "shoot" => Some (MessageType::Shoot),
            "quaff" => Some (MessageType::Quaff),
            "zap_rod" => Some (MessageType::ZapRod),
            "walk" => Some (MessageType::Walk),
            "tpother" => Some (MessageType::TPOther),
            "hitwall" => Some (MessageType::HitWall),
            "eat" => Some (MessageType::Eat),
            "store1" => Some (MessageType::Store1),
            "store2" => Some (MessageType::Store2),
            "store3" => Some (MessageType::Store3),
            "store4" => Some (MessageType::Store4),
            "dig" => Some (MessageType::Dig),
            "opendoor" => Some (MessageType::OpenDoor),
            "shutdoor" => Some (MessageType::ShutDoor),
            "tplevel" => Some (MessageType::TPLevel),
            "bell" => Some (MessageType::Bell),
            "nothing_to_open" => Some (MessageType::NothingToOpen),
            "lockpick_fail" => Some (MessageType::LockpickFail),
            "stairs_down" => Some (MessageType::StairsDown),
            "hitpoint_warn" => Some (MessageType::HitpointWarn),
            "act_artifact" => Some (MessageType::ActArtifact),
            "use_staff" => Some (MessageType::UseStaff),
            "destroy" => Some (MessageType::Destroy),
            "mon_hit" => Some (MessageType::MonHit),
            "mon_touch" => Some (MessageType::MonTouch),
            "mon_punch" => Some (MessageType::MonPunch),
            "mon_kick" => Some (MessageType::MonKick),
            "mon_claw" => Some (MessageType::MonClaw),
            "mon_bite" => Some (MessageType::MonBite),
            "mon_sting" => Some (MessageType::MonSting),
            "mon_butt" => Some (MessageType::MonButt),
            "mon_crush" => Some (MessageType::MonCrush),
            "mon_engulf" => Some (MessageType::MonEngulf),
            "mon_crawl" => Some (MessageType::MonCrawl),
            "mon_drool" => Some (MessageType::MonDrool),
            "mon_spit" => Some (MessageType::MonSpit),
            "mon_gaze" => Some (MessageType::MonGaze),
            "mon_wail" => Some (MessageType::MonWail),
            "mon_spore" => Some (MessageType::MonSpore),
            "mon_beg" => Some (MessageType::MonBeg),
            "mon_insult" => Some (MessageType::MonInsult),
            "mon_moan" => Some (MessageType::MonMoan),
            "recover" => Some (MessageType::Recover),
            "blind" => Some (MessageType::Blind),
            "confused" => Some (MessageType::Confused),
            "poisoned" => Some (MessageType::Poisoned),
            "afraid" => Some (MessageType::Afraid),
            "paralyzed" => Some (MessageType::Paralyzed),
            "drugged" => Some (MessageType::Drugged),
            "speed" => Some (MessageType::Speed),
            "slow" => Some (MessageType::Slow),
            "shield" => Some (MessageType::Shield),
            "blessed" => Some (MessageType::Blessed),
            "hero" => Some (MessageType::Hero),
            "berserk" => Some (MessageType::Berserk),
            "bold" => Some (MessageType::Bold),
            "prot_evil" => Some (MessageType::ProtEvil),
            "invuln" => Some (MessageType::Invuln),
            "see_invis" => Some (MessageType::SeeInvis),
            "infrared" => Some (MessageType::Infrared),
            "res_acid" => Some (MessageType::ResAcid),
            "res_elec" => Some (MessageType::ResElec),
            "res_fire" => Some (MessageType::ResFire),
            "res_cold" => Some (MessageType::ResCold),
            "res_pois" => Some (MessageType::ResPois),
            "stun" => Some (MessageType::Stun),
            "cut" => Some (MessageType::Cut),
            "stairs_up" => Some (MessageType::StairsUp),
            "store_enter" => Some (MessageType::StoreEnter),
            "store_leave" => Some (MessageType::StoreLeave),
            "store_home" => Some (MessageType::StoreHome),
            "money1" => Some (MessageType::Money1),
            "money2" => Some (MessageType::Money2),
            "money3" => Some (MessageType::Money3),
            "shoot_hit" => Some (MessageType::ShootHit),
            "stores" => Some (MessageType::Stores),
            "lockpick" => Some (MessageType::Lockpick),
            "disarm" => Some (MessageType::Disarm),
            "ident_bad" | "identify_bad" => Some (MessageType::IdentBad),
            "ident_ego" | "identify_ego" => Some (MessageType::IdentEgo),
            "ident_art" | "idnetify_art" => Some (MessageType::IdentArt),
            "br_elements" | "breathe_elements" => Some (MessageType::BrElements),
            "br_frost" | "breathe_frost" => Some (MessageType::BrFrost),
            "br_elec" | "breathe_elec" => Some (MessageType::BrElec),
            "br_acid" | "breathe_acid" => Some (MessageType::BrAcid),
            "br_gas" | "breathe_gas" => Some (MessageType::BrGas),
            "br_fire" | "breathe_fire" => Some (MessageType::BrFire),
            "br_disen" | "breathe_disenchant" => Some (MessageType::BrDisen),
            "br_chaos" | "breahte_chaos" => Some (MessageType::BrChaos),
            "br_shards" | "breathe_shards" => Some (MessageType::BrShards),
            "br_sound" | "breathe_sound" => Some (MessageType::BrSound),
            "br_light" | "breathe_light" => Some (MessageType::BrLight),
            "br_dark" | "breathe_dark" => Some (MessageType::BrDark),
            "br_nether" | "breathe_nether" => Some (MessageType::BrNether),
            "br_nexus" | "breathe_nexus" => Some (MessageType::BrNexus),
            "br_time" | "breathe_time" => Some (MessageType::BrTime),
            "br_inertia" | "breathe_ineertia" => Some (MessageType::BrInertia),
            "br_gravity" | "breathe_gravity" => Some (MessageType::BrGravity),
            "br_plasma" | "breathe_plasma" => Some (MessageType::BrPlasma),
            "br_force" | "breathe_force" => Some (MessageType::BrForce),
            "sum_monster" | "summon_monsters" => Some (MessageType::SumMonster),
            "sum_ainu" | "summon_ainu" => Some (MessageType::SumAinu),
            "sum_undead" | "summon_undead" => Some (MessageType::SumUndead),
            "sum_animal" | "summon_animal" => Some (MessageType::SumAnimal),
            "sum_spider" | "summon_spider" => Some (MessageType::SumSpider),  
            "sum_hound" | "summon_hound" => Some (MessageType::SumHound),
            "sum_hydra" | "summon_hydra" => Some (MessageType::SumHydra),
            "sum_demon" | "summon_demon" => Some (MessageType::SumDemon),
            "sum_dragon" | "summon_dragon" => Some (MessageType::SumDragon),
            "sum_hi_undead" | "summon_gr_undead" => Some (MessageType::SumHiUndead),
            "sum_hi_dragon" | "summon_gr_dragon" => Some (MessageType::SumHiDragon),
            "sum_hi_demon" | "summon_gr_demon" => Some (MessageType::SumHiDemon),
            "sum_wraith" | "summon_ringwraith" => Some (MessageType::SumWraith),
            "sum_unique" | "summon_unique" => Some (MessageType::SumUnique),
            "wield" => Some (MessageType::Wield),
            "quiver" => Some (MessageType::Quiver),
            "cursed" => Some (MessageType::Cursed),
            "rune" => Some (MessageType::Rune),
            "hungry" => Some (MessageType::Hungry),
            "notice" => Some (MessageType::Notice),
            "ambient_day" => Some (MessageType::AmbientDay),
            "ambient_nite" => Some (MessageType::AmbientNite),
            "ambient_dng1" => Some (MessageType::AmbientDng1),
            "ambient_dng2" => Some (MessageType::AmbientDng2),
            "ambient_dng3" => Some (MessageType::AmbientDng3),
            "ambient_dng4" => Some (MessageType::AmbientDng4),
            "ambient_dng5" => Some (MessageType::AmbientDng5),
            "create_trap" | "mon_create_trap" => Some (MessageType::CreateTrap),
            "shriek" | "mon_shriek" => Some (MessageType::Shriek),
            "cast_fear" | "mon_cast_fear" => Some (MessageType::CastFear),
            "hit_good" => Some (MessageType::HitGood),
            "hit_great" => Some (MessageType::HitGreat),
            "hit_superb" => Some (MessageType::HitSuperb),
            "hit_hi_great" => Some (MessageType::HitHiGreat),
            "hit_hi_superb" => Some (MessageType::HitHiSuperb),
            "spell" | "cast_spell" => Some (MessageType::Spell),
            "prayer" | "pray_prayer" => Some (MessageType::Prayer),
            "kill_unique" => Some (MessageType::KillUnique),
            "kill_king" => Some (MessageType::KillKing),
            "drain_stat" => Some (MessageType::DrainStat),
            "multiply" => Some (MessageType::Multiply),
            "scramble" => Some (MessageType::Scramble),
            _ => None
        }
    }

    pub fn sound_name(m_type: &MessageType) -> &str {
        match m_type {
            MessageType::Generic => "generic",
            MessageType::Birth => "birth",
            MessageType::Hit => "hit",
            MessageType::Miss => "miss",
            MessageType::Flee => "flee",
            MessageType::Drop => "drop",
            MessageType::Kill => "kill",
            MessageType::Level => "level",
            MessageType::Death => "death",
            MessageType::Study => "study",
            MessageType::Teleport => "teleport",
            MessageType::Shoot => "shoot",
            MessageType::Quaff => "quaff",
            MessageType::ZapRod => "zap_rod",
            MessageType::Walk => "walk",
            MessageType::TPOther => "tpother",
            MessageType::HitWall => "hitwall",
            MessageType::Eat => "eat",
            MessageType::Store1 => "store1",
            MessageType::Store2 => "store2",
            MessageType::Store3 => "store3",
            MessageType::Store4 => "store4",
            MessageType::Dig => "dig",
            MessageType::OpenDoor => "opendoor",
            MessageType::ShutDoor => "shutdoor",
            MessageType::TPLevel => "tplevel",
            MessageType::Bell => "bell",
            MessageType::NothingToOpen => "nothing_to_open",
            MessageType::LockpickFail => "lockpick_fail",
            MessageType::StairsDown => "stairs_down",
            MessageType::HitpointWarn => "hitpoint_warn",
            MessageType::ActArtifact => "act_artifact",
            MessageType::UseStaff => "use_staff",
            MessageType::Destroy => "destroy",
            MessageType::MonHit => "mon_hit",
            MessageType::MonTouch => "mon_touch",
            MessageType::MonPunch => "mon_punch",
            MessageType::MonKick => "mon_kick",
            MessageType::MonClaw => "mon_claw",
            MessageType::MonBite => "mon_bite",
            MessageType::MonSting => "mon_sting",
            MessageType::MonButt => "mon_butt",
            MessageType::MonCrush => "mon_crush",
            MessageType::MonEngulf => "mon_engulf",
            MessageType::MonCrawl => "mon_crawl",
            MessageType::MonDrool => "mon_drool",
            MessageType::MonSpit => "mon_spit",
            MessageType::MonGaze => "mon_gaze",
            MessageType::MonWail => "mon_wail",
            MessageType::MonSpore => "mon_spore",
            MessageType::MonBeg => "mon_beg",
            MessageType::MonInsult => "mon_insult",
            MessageType::MonMoan => "mon_moan",
            MessageType::Recover => "recover",
            MessageType::Blind => "blind",
            MessageType::Confused => "confused",
            MessageType::Poisoned => "poisoned",
            MessageType::Afraid => "afraid",
            MessageType::Paralyzed => "paralyzed",
            MessageType::Drugged => "drugged",
            MessageType::Speed => "speed",
            MessageType::Slow => "slow",
            MessageType::Shield => "shield",
            MessageType::Blessed => "blessed",
            MessageType::Hero => "hero",
            MessageType::Berserk => "berserk",
            MessageType::Bold => "bold",
            MessageType::ProtEvil => "prot_evil",
            MessageType::Invuln => "invuln",
            MessageType::SeeInvis => "see_invis",
            MessageType::Infrared => "infrared",
            MessageType::ResAcid => "res_acid",
            MessageType::ResElec => "res_elec",
            MessageType::ResFire => "res_fire",
            MessageType::ResCold => "res_cold",
            MessageType::ResPois => "res_pois",
            MessageType::Stun => "stun",
            MessageType::Cut => "cut",
            MessageType::StairsUp => "stairs_up",
            MessageType::StoreEnter => "store_enter",
            MessageType::StoreLeave => "store_leave",
            MessageType::StoreHome => "store_home",
            MessageType::Money1 => "money1",
            MessageType::Money2 => "money2",
            MessageType::Money3 => "money3",
            MessageType::ShootHit => "shoot_hit",
            MessageType::Stores => "stores",
            MessageType::Lockpick => "lockpick",
            MessageType::Disarm => "disarm",
            MessageType::IdentBad => "identify_bad",
            MessageType::IdentEgo => "identify_ego",
            MessageType::IdentArt => "identify_art",
            MessageType::BrElements => "breathe_elements",
            MessageType::BrFrost => "breathe_frost",
            MessageType::BrElec => "breathe_elec",
            MessageType::BrAcid => "breathe_acid",
            MessageType::BrGas => "breathe_gas",
            MessageType::BrFire => "breathe_fire",
            MessageType::BrDisen => "breathe_disenchant",
            MessageType::BrChaos => "breathe_chaos",
            MessageType::BrShards => "breathe_shards",
            MessageType::BrSound => "breathe_sound",
            MessageType::BrLight => "breathe_light",
            MessageType::BrDark => "breathe_dark",
            MessageType::BrNether => "breathe_nether",
            MessageType::BrNexus => "breathe_nexus",
            MessageType::BrTime => "breathe_time",
            MessageType::BrInertia => "breathe_inertia",
            MessageType::BrGravity => "breathe_gravity",
            MessageType::BrPlasma => "breathe_plasma",
            MessageType::BrForce => "breathe_force",
            MessageType::SumMonster => "summon_monsters",
            MessageType::SumAinu => "summon_ainu",
            MessageType::SumUndead => "summon_undead",
            MessageType::SumAnimal => "summon_animal",
            MessageType::SumSpider => "summon_spider",  
            MessageType::SumHound => "summon_hound",
            MessageType::SumHydra => "summon_hydra",
            MessageType::SumDemon => "summon_demon",
            MessageType::SumDragon => "summon_dragon",
            MessageType::SumHiUndead => "summon_gr_undead",
            MessageType::SumHiDragon => "summon_gr_dragon",
            MessageType::SumHiDemon => "summon_gr_demon",
            MessageType::SumWraith => "summon_ringwraith",
            MessageType::SumUnique => "summon_unique",
            MessageType::Wield => "wield",
            MessageType::Quiver => "quiver",
            MessageType::Cursed => "cursed",
            MessageType::Rune => "rune",
            MessageType::Hungry => "hungry",
            MessageType::Notice => "notice",
            MessageType::AmbientDay => "ambient_day",
            MessageType::AmbientNite => "ambient_nite",
            MessageType::AmbientDng1 => "ambient_dng1",
            MessageType::AmbientDng2 => "ambient_dng2",
            MessageType::AmbientDng3 => "ambient_dng3",
            MessageType::AmbientDng4 => "ambient_dng4",
            MessageType::AmbientDng5 => "ambient_dng5",
            MessageType::CreateTrap => "mon_create_trap",
            MessageType::Shriek => "mon_shriek",
            MessageType::CastFear => "mon_cast_fear",
            MessageType::HitGood => "hit_good",
            MessageType::HitGreat => "hit_great",
            MessageType::HitSuperb => "hit_superb",
            MessageType::HitHiGreat => "hit_hi_great",
            MessageType::HitHiSuperb => "hit_hi_superb",
            MessageType::Spell => "cast_spell",
            MessageType::Prayer => "pray_prayer",
            MessageType::KillUnique => "kill_unique",
            MessageType::KillKing => "kill_king",
            MessageType::DrainStat => "drain_stat",
            MessageType::Multiply => "multiply",
            MessageType::Scramble => "scramble"
        }
    }
}

pub struct Message {
    pub str: String,
    pub m_type: MessageType,
    pub count: u16
}

pub struct MessageService {
    pub messages: VecDeque<Message>,
    pub colors: HashMap<MessageType, Colors>,
    pub max: u32
}

impl MessageService {
    pub fn init() -> MessageService {
        MessageService {
            messages: VecDeque::<Message>::new(),
            colors: HashMap::<MessageType, Colors>::new(),
            max: 2048
        }
    }

    pub fn add(&mut self, str: &str, m_type: &MessageType) -> () {
        let head = self.messages.pop_front();
        match head {
            Some (mut head) => {
                if head.m_type == *m_type && head.str == str {
                    head.count += 1;
                    self.messages.push_front(head);
                } else {
                    let m = Message {
                        str: String::from(str),
                        m_type: *m_type,
                        count: 1
                    };
                    self.messages.push_front(head);
                    self.messages.push_front(m);
                }
            },
            None => {
                let m = Message {
                    str: String::from(str),
                    m_type: *m_type,
                    count: 1
                };
                self.messages.push_front(m);
            }
        }
    }

    pub fn get(&self, age: usize) -> Option<&Message> {
        self.messages.get(age + 1)
    }

    pub fn str(&self, age: usize) -> &str {
        match self.messages.get(age + 1) {
            Some (m) => &m.str,
            None => ""
        }
    }

    pub fn count(&self, age: usize) -> &u16 {
        match self.messages.get(age + 1) {
            Some (m) => &m.count,
            None => &0
        }
    }

    pub fn m_type(&self, age: usize) -> &MessageType {
        match self.messages.get(age + 1) {
            Some (m) => &m.m_type,
            None => &MessageType::Generic
        }
    }

    pub fn color(&self, age: usize) -> &Colors {
        match self.messages.get(age + 1) {
            Some (m) => self.type_color(&m.m_type),
            None => &Colors::White
        }
    }

    pub fn type_color(&self, m_type: &MessageType) -> &Colors {
        match self.colors.get(m_type) {
            Some (c) => &c,
            None => &Colors::White
        }
    }

    pub fn color_define(&mut self, m_type: &MessageType, color: &Colors) -> () {
        if self.colors.contains_key(m_type) {
            self.colors.remove(m_type);
        }
        self.colors.insert(*m_type, *color);
    }

    pub fn sound(queue: &mut MessageQueue, m_type: &MessageType) -> () {
        queue.dispatch(&GameEvent::EventSound, GameEventData::Message (EvMessage { msg: None, msg_type: *m_type }))
    }

    pub fn bell(queue: &mut MessageQueue) -> () {
        queue.dispatch(&GameEvent::EventBell, GameEventData::Message (EvMessage { msg: None, msg_type: MessageType::Bell }))
    }

    pub fn msg(&mut self, queue: &mut MessageQueue, msg: &str) -> () {
        self.add(msg, &MessageType::Generic);
        queue.dispatch(&GameEvent::EventMessage, GameEventData::Message (EvMessage { msg: Some (String::from(msg)), msg_type: MessageType::Generic }))
    }

    pub fn msgt(&mut self, queue: &mut MessageQueue, m_type: &MessageType, msg: &str) -> () {
        self.add(msg, m_type);
        MessageService::sound(queue, m_type);
        queue.dispatch(&GameEvent::EventMessage, GameEventData::Message (EvMessage { msg: Some (String::from(msg)), msg_type: *m_type }))
    }
}