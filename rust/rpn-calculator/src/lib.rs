use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn binary_op(stack: &mut Vec<i32>, f: impl Fn(i32, i32) -> i32) -> Option<i32> {
    stack
        .pop()
        .and_then(|y| stack.pop().and_then(|x| Some(f(x, y))))
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let v: Vec<i32> = vec![];
    let mut result = inputs.iter().fold(v, |mut stack, input| {
        if let Some(new) = match input {
            CalculatorInput::Add => binary_op(&mut stack, i32::add),
            CalculatorInput::Subtract => binary_op(&mut stack, i32::sub),
            CalculatorInput::Multiply => binary_op(&mut stack, i32::mul),
            CalculatorInput::Divide => binary_op(&mut stack, i32::div),
            CalculatorInput::Value(value) => Some(*value),
        } {
            stack.push(new);
        }
        stack
    });
    result
        .pop()
        .and_then(|x| if result.is_empty() { Some(x) } else { None })
}
