pub enum RuneVariety {
    Combat,
    Mod,
    Resist,
    Brand,
    Slay,
    Curse,
    Flag,
}

pub enum CombatRunes {
    ToA,
    ToH,
    ToD,
}

pub struct Rune {
    pub variety: RuneVariety,
    pub index: i32,
    pub note: String,
    pub name: String,
}

pub struct RuneService {
    pub rune_list: Vec<Rune>,
}
