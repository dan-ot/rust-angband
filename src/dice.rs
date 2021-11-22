// Dice in this context is the representation of a potential roll. They are not involved in the actual randomness -
// that comes from RandomValue from the Random module. This part 

use crate::random::Aspect;
use crate::random::Random;
use crate::random::Diceroll;
use std::collections::HashMap;
use crate::expressions::Expression;

#[derive(Debug, Clone)]
pub enum ExpressionOrValue {
    Expression (Expression),
    Value (i32)
}

#[derive(Debug, Clone)]
pub struct Dice {
    /// Base (non-random) number
    b: ExpressionOrValue,
    /// Number of dice in the roll
    x: ExpressionOrValue,
    /// Number of sides per die
    y: ExpressionOrValue,
    /// Bonus on the roll
    m: ExpressionOrValue
}

pub enum DiceState {
    Start,      // A
    BaseDigit,  // B
    FlushBase,  // C
    DiceDigit,  // D
    FlushDice,  // E
    SideDigit,  // F
    FlushSide,  // G
    Bonus,      // H
    BonusDigit, // I
    FlushBonus, // J
    Var,        // K
    VarChar,    // L
    FlushAll,   // M
    /// We're done
    Max         // N, .  
}

pub enum DiceInput {
    Amp,        // &
    Minus,      // -
    Base,       // +
    Dice,       // d
    Bonus,      // M, m
    Var,        // $
    Digit,      // D
    Upper,      // U
    // TODO: This is null-terminated strings?
    Null,       // 0
    // TODO: Max == Error?
    Max
}

fn input_for_char(c: &char) -> DiceInput {
    match c {
        '&' => DiceInput::Amp,
        '-' => DiceInput::Minus,
        '+' => DiceInput::Base,
        'd' => DiceInput::Dice,
        'M' | 'm' => DiceInput::Bonus,
        '$' => DiceInput::Var,
        '\0' => DiceInput::Null,
        '0'..='9' => DiceInput::Digit,
        'A'..='Z' => DiceInput::Upper,
        _ => DiceInput::Max
    }
}

fn state_transition(state: &DiceState, input: &DiceInput) -> DiceState {
    match state {
        DiceState::Start => match input {
            DiceInput::Minus => DiceState::BaseDigit,
            DiceInput::Dice => DiceState::FlushDice,
            DiceInput::Bonus => DiceState::Bonus,
            DiceInput::Var => DiceState::Var,
            DiceInput::Digit => DiceState::BaseDigit,
            _ => DiceState::Max
        },
        DiceState::BaseDigit => match input {
            DiceInput::Base => DiceState::FlushBase,
            DiceInput::Dice => DiceState::FlushDice,
            DiceInput::Digit => DiceState::BaseDigit,
            DiceInput::Null => DiceState::FlushBase,
            _ => DiceState::Max
        },
        DiceState::FlushBase => match input {
            DiceInput::Dice => DiceState::FlushDice,
            DiceInput::Bonus => DiceState::Bonus,
            DiceInput::Var => DiceState::Var,
            DiceInput::Digit => DiceState::DiceDigit,
            _ => DiceState::Max
        },
        DiceState::DiceDigit => match input {
            DiceInput::Dice => DiceState::FlushDice,
            DiceInput::Digit => DiceState::DiceDigit,
            _ => DiceState::Max
        },
        DiceState::FlushDice => match input {
            DiceInput::Var => DiceState::Var,
            DiceInput::Digit => DiceState::SideDigit,
            _ => DiceState::Max
        },
        DiceState::SideDigit => match input {
            DiceInput::Amp => DiceState::FlushSide,
            DiceInput::Bonus => DiceState::Bonus,
            DiceInput::Digit => DiceState::SideDigit,
            DiceInput::Null => DiceState::FlushSide,
            _ => DiceState::Max
        },
        DiceState::FlushSide => match input {
            DiceInput::Bonus => DiceState::Bonus,
            _ => DiceState::Max
        },
        DiceState::Bonus => match input {
            DiceInput::Var => DiceState::Var,
            DiceInput::Digit => DiceState::BonusDigit,
            _ => DiceState::Max
        },
        DiceState::BonusDigit => match input {
            DiceInput::Digit => DiceState::BonusDigit,
            DiceInput::Null => DiceState::FlushBonus,
            _ => DiceState::Max
        },
        DiceState::FlushBonus => match input {
            _ => DiceState::Max
        },
        DiceState::Var => match input {
            DiceInput::Upper => DiceState::VarChar,
            _ => DiceState::Max
        },
        DiceState::VarChar => match input {
            DiceInput::Amp => DiceState::FlushSide,
            DiceInput::Base => DiceState::FlushBase,
            DiceInput::Dice => DiceState::FlushDice,
            DiceInput::Bonus => DiceState::Bonus,
            DiceInput::Upper => DiceState::VarChar,
            DiceInput::Null => DiceState::FlushAll,
            _ => DiceState::Max
        },
        DiceState::FlushAll
        | DiceState::Max => DiceState::Max
    }
}

impl Dice {
    pub fn new() -> Dice {
        Dice {
            b: ExpressionOrValue::Value (0),
            x: ExpressionOrValue::Value (0),
            m: ExpressionOrValue::Value (0),
            y: ExpressionOrValue::Value (0)
        }
    }

    pub fn reset(&mut self) -> () {
        self.b = ExpressionOrValue::Value (0);
        self.x = ExpressionOrValue::Value (0);
        self.m = ExpressionOrValue::Value (0);
        self.y = ExpressionOrValue::Value (0);
    }

    // /// Adds an expression under a given name. Original returned an int index,
    // /// but we're using a Map here so we couldn't make use of indices.
    // pub fn bind_expression(&mut self, name: String, expression: Expression) -> () {
    //     self.expressions.insert(name, expression);
    // }

    /// Load the Dice with the values from the input. Returns true if successful.
    pub fn parse_string(&mut self, string: &str) -> bool {
        let mut state = DiceState::Start;
        let mut token: Vec<char> = vec!();

        if string.is_empty() {
            return false;
        }
        
        self.reset();

        for (current, ch) in string.char_indices() {
            let mut input_type = DiceInput::Max;
            let mut last_seen = LastSeen::None;

            if ch.is_whitespace() {
                continue;
            }

            input_type = input_for_char(&ch);

            state = match input_type {
                DiceInput::Amp
                | DiceInput::Base
                | DiceInput::Dice
                | DiceInput::Var
                | DiceInput::Null => {
                    state_transition(&state, &input_type)
                },

                DiceInput::Minus
                | DiceInput::Digit
                | DiceInput::Upper => {
                    token.push(ch);
                    state_transition(&state, &input_type)
                },

                _ => state
            };

            state = match ch {
                'M' => match state {
                    DiceState::Var | DiceState::VarChar => {
                        token.push(ch);
                        state_transition(&state, &DiceInput::Upper)
                    },
                    _ => state_transition(&state, &DiceInput::Bonus)
                },
                'm' => state_transition(&state, &DiceInput::Bonus),
                _ => state
            };

            let flush = match state {
                DiceState::FlushBase => {
                    last_seen = LastSeen::Base;
                    true
                },
                DiceState::FlushDice => {
                    last_seen = LastSeen::Dice;
                    true
                },
                DiceState::FlushSide => {
                    last_seen = LastSeen::Side;
                    true
                },
                DiceState::FlushBonus => {
                    last_seen = LastSeen::Bonus;
                    true
                },
                DiceState::FlushAll => {
                    last_seen = match last_seen {
                        LastSeen::None => LastSeen::Base,
                        LastSeen::Base => LastSeen::Dice,
                        LastSeen::Dice => LastSeen::Side,
                        LastSeen::Side => LastSeen::Bonus,
                        LastSeen::Bonus => LastSeen::Bonus
                    };
                    true
                },
                DiceState::Bonus => {
                    last_seen = if last_seen == LastSeen::Dice { LastSeen::Side } else { LastSeen::Bonus };
                    true
                }
                _ => false
            };

            if flush && token.len() > 0 {
                let as_str = token.iter().collect::<String>();
                let value = match as_str.parse::<i32>() {  
                    Ok (val) => ExpressionOrValue::Value (val),
                    Err (_) => {
                        let mut exp = Expression::new();
                        exp.append_operations_string(as_str.as_str());
                        ExpressionOrValue::Expression (exp)
                    }
                };

                match last_seen {
                    LastSeen::Base => {
                        self.b = value;
                    },
                    LastSeen::Dice => {
                        self.x = value;
                    },
                    LastSeen::Side => {
                        self.y = value;
                    },
                    LastSeen::Bonus => {
                        self.m = value;
                    },
                    LastSeen::None => ()
                }

                token.clear();
            }
        }

        true
    }

    /// Resolve the expressions and values of the Dice into a rollable form
    pub fn random_value(&self) -> Diceroll {
        Diceroll::new(
            match &self.b {
                ExpressionOrValue::Value (v) => *v,
                ExpressionOrValue::Expression (e) => e.evaluate()
            },
            match &self.x {
                ExpressionOrValue::Value (v) => *v,
                ExpressionOrValue::Expression (e) => e.evaluate()
            },
            match &self.y {
                ExpressionOrValue::Value (v) => *v,
                ExpressionOrValue::Expression (e) => e.evaluate()
            },
            match &self.m {
                ExpressionOrValue::Value (v) => *v,
                ExpressionOrValue::Expression (e) => e.evaluate()
            }
        )
    }

    /// Use the provided Random to make the expressed dice roll
    pub fn evaluate(&self, rng: &mut Random, level: i32, aspect: &Aspect) -> i32 {
        let roll = self.random_value();
        roll.resolve(rng, level, aspect)
    }

    /// Do a (simpler) damage roll using the provided Random
    pub fn roll(&self, rng: &mut Random) -> i32 {
        let roll = self.random_value();

        roll.base + rng.damroll(roll.dice, roll.sides)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum LastSeen {
    None,
    Base,
    Dice,
    Side,
    Bonus
}