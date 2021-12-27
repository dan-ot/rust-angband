pub enum RuneVariety {
    Combat,
    Mod,
    Resist,
    Brand,
    Slay,
    Curse,
    Flag
}

pub enum CombatRunes {
    ToA,
    ToH,
    ToD,
    Max
}

pub struct Rune {
    pub variety: RuneVariety,
    pub index: i32,
    // TODO: Quark?
    pub note: String,
    pub name: String
}