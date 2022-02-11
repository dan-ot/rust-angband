#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Stats {
    Str,
    Int,
    Wis,
    Dex,
    Con,
}

impl Stats {
    pub fn name(&self) -> &str {
        match self {
            Stats::Str => "STR",
            Stats::Int => "INT",
            Stats::Wis => "WIS",
            Stats::Dex => "DEX",
            Stats::Con => "CON",
        }
    }

    pub fn try_parse(input: &str) -> Option<Stats> {
        match input.to_lowercase().as_str() {
            "str" => Some(Stats::Str),
            "int" => Some(Stats::Int),
            "wis" => Some(Stats::Wis),
            "dex" => Some(Stats::Dex),
            "con" => Some(Stats::Con),
            _ => None,
        }
    }
}
