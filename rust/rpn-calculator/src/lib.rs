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
    stack.pop().and_then(|y| stack.pop().map(|x| f(x, y)))
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    inputs
        .iter()
        .try_fold(vec![], |mut stack, input| {
            match input {
                CalculatorInput::Add => binary_op(&mut stack, i32::add),
                CalculatorInput::Subtract => binary_op(&mut stack, i32::sub),
                CalculatorInput::Multiply => binary_op(&mut stack, i32::mul),
                CalculatorInput::Divide => binary_op(&mut stack, i32::div),
                CalculatorInput::Value(value) => Some(*value),
            }
            .map(|x| {
                stack.push(x);
                stack
            })
        })
        .and_then(|stack| match stack.as_slice() {
            [x] => Some(*x),
            _ => None,
        })
}
