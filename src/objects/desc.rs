use crate::bitflags::Bitflag;

pub enum DescMode {
    Base,
    Combat,
    Extra,
    Store,
    Plural,
    Singular,
    Spoil,
    Prefix,
    Capital,
    Terse,
    NoEgo,
    Max,
}

impl From<DescMode> for usize {
    fn from(d: DescMode) -> Self {
        d as usize
    }
}

impl DescMode {
    pub fn full() -> Bitflag {
        let mut b = Bitflag::new(DescMode::Max as usize);
        b.init(Box::new(
            vec![DescMode::Combat, DescMode::Extra].into_iter(),
        ));
        b
    }
}

enum CustomMode {
    Singular(String, String),
    Plural(String, String),
}

enum FormatMode {
    None(String),
    Skip(String),
    Custom(CustomMode),
}

pub fn obj_desc_name_format(fmt: &str, modstr: Option<&str>, pluralize: bool) -> String {
    let mut mode = FormatMode::None(String::default());
    for c in fmt.chars() {
        match mode {
            FormatMode::None(stored) => match c {
                '&' => {
                    mode = FormatMode::Skip(stored);
                }
                '|' => {
                    mode = FormatMode::Custom(CustomMode::Singular(String::default(), stored));
                }
                '#' => match modstr {
                    Some(m) => {
                        let modded = obj_desc_name_format(m, None, pluralize);
                        mode = FormatMode::None(stored + &modded);
                    }
                    None => {
                        mode = FormatMode::None(stored);
                    }
                },
                _ => {
                    mode = FormatMode::None(stored + &(c.to_string()));
                }
            },
            FormatMode::Custom(CustomMode::Singular(so_far, stored)) => match c {
                '|' => {
                    let new_stored = if pluralize == false {
                        stored + &so_far
                    } else {
                        stored.clone()
                    };
                    mode = FormatMode::Custom(CustomMode::Plural(String::default(), new_stored));
                }
                _ => {
                    mode =
                        FormatMode::Custom(CustomMode::Singular(so_far + &(c.to_string()), stored));
                }
            },
            FormatMode::Custom(CustomMode::Plural(so_far, stored)) => match c {
                '|' => {
                    let new_stored = if pluralize == true {
                        stored + &so_far
                    } else {
                        stored.clone()
                    };
                    mode = FormatMode::None(new_stored);
                }
                _ => {
                    mode =
                        FormatMode::Custom(CustomMode::Plural(so_far + &(c.to_string()), stored));
                }
            },
            FormatMode::Skip(stored) => match c {
                '&' | ' ' => {
                    mode = FormatMode::Skip(stored);
                }
                _ => {
                    mode = FormatMode::None(stored + &(c.to_string()));
                }
            },
        }
    }
    String::from("")
}
