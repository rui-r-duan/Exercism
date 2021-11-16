use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

/// Returns (first_operand, second_operand)
fn extract_operands(stack: &mut Vec<i32>) -> (Option<i32>, Option<i32>) {
    let b = match stack.pop() {
        Some(x) => Some(x),
        None => None,
    };
    let a = match stack.pop() {
        Some(x) => Some(x),
        None => None,
    };
    (a, b)
}

fn calc(op_a: Option<i32>, op_b: Option<i32>, stack: &mut Vec<i32>, f: impl Fn(i32, i32) -> i32) {
    match op_a {
        Some(a) => match op_b {
            Some(b) => {
                let c = f(a, b);
                stack.push(c);
            }
            None => (),
        },
        None => (),
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    for token in inputs {
        match token {
            CalculatorInput::Add => {
                let (op_a, op_b) = extract_operands(&mut stack);
                calc(op_a, op_b, &mut stack, i32::add);
            }
            CalculatorInput::Subtract => {
                let (op_a, op_b) = extract_operands(&mut stack);
                calc(op_a, op_b, &mut stack, i32::sub);
            }
            CalculatorInput::Multiply => {
                let (op_a, op_b) = extract_operands(&mut stack);
                calc(op_a, op_b, &mut stack, i32::mul);
            }
            CalculatorInput::Divide => {
                let (op_a, op_b) = extract_operands(&mut stack);
                calc(op_a, op_b, &mut stack, i32::div);
            }
            CalculatorInput::Value(v) => {
                stack.push(*v);
            }
        }
    }
    match stack.as_slice() {
        [x] => Some(*x),
        _ => None,
    }
}
