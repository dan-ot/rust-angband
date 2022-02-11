/// These are flags for object prototypes, not individual objects
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum KindFlags {
    None,
    RandHiRes,
    RandSustain,
    RandPower,
    InstaArt,
    QuestArt,
    EasyKnow,
    Good,
    ShowDice,
    ShowMult,
    ShootsShots,
    ShootsArrows,
    ShootsBolts,
    RandBaseRes,
    RandResPower,
}
