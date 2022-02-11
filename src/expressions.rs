// The Expression module handles parsing strings into evaluatable expression trees
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ExpressionOperator {
    None,
    Add,
    Sub,
    Mul,
    Div,
    Neg,
}

pub fn operator_from_token(token: &str) -> ExpressionOperator {
    match token {
        "+" => ExpressionOperator::Add,
        "-" => ExpressionOperator::Sub,
        "*" => ExpressionOperator::Mul,
        "/" => ExpressionOperator::Div,
        "n" | "N" => ExpressionOperator::Neg,
        _ => ExpressionOperator::None,
    }
}

pub fn input_for_operator(operator: &ExpressionOperator) -> ExpressionParserInput {
    match operator {
        ExpressionOperator::None => ExpressionParserInput::Invalid,
        ExpressionOperator::Add
        | ExpressionOperator::Sub
        | ExpressionOperator::Mul
        | ExpressionOperator::Div => ExpressionParserInput::NeedsOperands,
        ExpressionOperator::Neg => ExpressionParserInput::UnaryOperator,
    }
}

// TODO: This should probably be Result<State> instead of magic negative values
#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum ExpressionParserState {
    ErrGeneric = -1,
    ErrInvalidOperator = -2,
    ErrExpectedOperator = -3,
    ErrExpectedOperand = -4,
    ErrDivideByZero = -5,
    Start = 0,
    Operator = 1,
    Operand = 2,
}

pub enum ExpressionParserInput {
    Invalid,
    NeedsOperands,
    UnaryOperator,
    Value,
}

const DELIMITER: &str = " ";

#[derive(Debug, Copy, Clone)]
pub struct ExpressionOperation {
    operator: ExpressionOperator,
    operand: i32,
}

pub struct Expression {
    /// Trouble! Because this is lazy, it's meant to capture context not available to the expression tree when it's built
    /// and provide that context to the expression...
    base_value: Option<Box<dyn Fn() -> i32>>,

    operations: Vec<ExpressionOperation>,
}

impl Debug for Expression {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        todo!()
    }
}

impl Clone for Expression {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Expression {
    pub fn new() -> Expression {
        Expression {
            base_value: None,
            operations: vec![],
        }
    }

    pub fn set_base_value(&mut self, function: Box<dyn Fn() -> i32>) -> () {
        self.base_value = Some(function);
    }

    pub fn evaluate(&self) -> i32 {
        self.operations.iter().fold(
            match &self.base_value {
                Some(f) => f(),
                None => 0,
            },
            |prev, op| match op.operator {
                ExpressionOperator::None => prev,
                ExpressionOperator::Add => prev + op.operand,
                ExpressionOperator::Div => prev / op.operand,
                ExpressionOperator::Mul => prev * op.operand,
                ExpressionOperator::Neg => -prev,
                ExpressionOperator::Sub => prev - op.operand,
            },
        )
    }

    pub fn append_operation(&mut self, operation: ExpressionOperation) -> () {
        self.operations.push(operation);
    }

    // TODO: This should be a Result<i32>, not magic numbers
    pub fn append_operations_string(&mut self, string: &str) -> i32 {
        if string.is_empty() {
            // No-op
            return 0;
        }

        let mut count = 0;
        let mut state = ExpressionParserState::Start;
        let mut parsed_operator = ExpressionOperator::None;
        let mut current_operator = ExpressionOperator::None;
        let mut current_input = ExpressionParserInput::Invalid;

        // TODO: this should probably be iterating over regex matches or something more specific
        for delimited in string.split(DELIMITER) {
            let chars = delimited
                .chars()
                .take_while(|c| c.is_digit(10))
                .collect::<String>();
            if chars.is_empty() {
                parsed_operator = operator_from_token(delimited);
                current_input = input_for_operator(&parsed_operator);
                state = next_state(&state, &current_input);
            } else {
                state = next_state(&state, &ExpressionParserInput::Value);
            }

            // TODO: Lack of C-ness is backfiring here. Need to code up an actual parser.
            let token = chars.as_str();
            let rest = delimited.get(chars.len()..);
            let value = token.parse::<i32>().unwrap();

            match state {
                ExpressionParserState::ErrDivideByZero
                | ExpressionParserState::ErrExpectedOperand
                | ExpressionParserState::ErrExpectedOperator
                | ExpressionParserState::ErrInvalidOperator
                | ExpressionParserState::ErrGeneric => {
                    return state as i32;
                }
                ExpressionParserState::Start => {
                    self.append_operation(ExpressionOperation {
                        operator: parsed_operator,
                        operand: 0,
                    });
                    count += 1;
                }
                ExpressionParserState::Operator => {
                    current_operator = parsed_operator;
                }
                ExpressionParserState::Operand => {
                    if current_operator == ExpressionOperator::Div && value == 0 {
                        return ExpressionParserState::ErrDivideByZero as i32;
                    }
                }
            }
        }

        count
    }
}

fn next_state(
    state: &ExpressionParserState,
    input: &ExpressionParserInput,
) -> ExpressionParserState {
    match state {
        ExpressionParserState::Start => match input {
            ExpressionParserInput::Invalid => ExpressionParserState::ErrInvalidOperator,
            ExpressionParserInput::NeedsOperands => ExpressionParserState::Operator,
            ExpressionParserInput::UnaryOperator => ExpressionParserState::Start,
            ExpressionParserInput::Value => ExpressionParserState::ErrExpectedOperator,
        },
        ExpressionParserState::Operator => match input {
            ExpressionParserInput::Invalid => ExpressionParserState::ErrInvalidOperator,
            ExpressionParserInput::NeedsOperands => ExpressionParserState::ErrExpectedOperand,
            ExpressionParserInput::UnaryOperator => ExpressionParserState::ErrExpectedOperand,
            ExpressionParserInput::Value => ExpressionParserState::Operand,
        },
        ExpressionParserState::Operand => match input {
            ExpressionParserInput::Invalid => ExpressionParserState::ErrInvalidOperator,
            ExpressionParserInput::NeedsOperands => ExpressionParserState::Operator,
            ExpressionParserInput::UnaryOperator => ExpressionParserState::Start,
            ExpressionParserInput::Value => ExpressionParserState::Operand,
        },
        // Passthrough for the various error states. Once errored, we're errored.
        other => *other,
    }
}
