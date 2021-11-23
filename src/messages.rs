use crate::colors::Colors;
use std::collections::HashMap;
use std::collections::VecDeque;

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

    pub fn add(&mut self, str: &str, m_type: MessageType) -> () {
        let head = self.messages.pop_front();
        match head {
            Some (mut head) => {
                if head.m_type == m_type && head.str == str {
                    head.count += 1;
                    self.messages.push_front(head);
                } else {
                    let m = Message {
                        str: String::from(str),
                        m_type: m_type,
                        count: 1
                    };
                    self.messages.push_front(head);
                    self.messages.push_front(m);
                }
            },
            None => {
                let m = Message {
                    str: String::from(str),
                    m_type: m_type,
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
            Some (m) => match self.colors.get(&m.m_type) {
                Some (c) => &c,
                None => &Colors::White
            },
            None => &Colors::White
        }
    }
}