#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

/// Returns (first_operand, second_operand)
fn extract_operands(stack: &mut Vec<CalculatorInput>) -> (Option<i32>, Option<i32>) {
    let b = match stack.pop() {
        Some(x) => match x {
            CalculatorInput::Value(v) => Some(v),
            _ => None,
        },
        None => None,
    };
    let a = match stack.pop() {
        Some(x) => match x {
            CalculatorInput::Value(v) => Some(v),
            _ => None,
        },
        None => None,
    };

    (a, b)
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<CalculatorInput> = Vec::new();
    for token in inputs {
        match token {
            CalculatorInput::Add => {
                let (op_a, op_b) = extract_operands(&mut stack);
                match op_a {
                    Some(a) => match op_b {
                        Some(b) => {
                            let c = a + b;
                            stack.push(CalculatorInput::Value(c));
                        }
                        None => {
                            return None;
                        }
                    },
                    None => {
                        return None;
                    }
                }
            }
            CalculatorInput::Subtract => {
                let (op_a, op_b) = extract_operands(&mut stack);
                match op_a {
                    Some(a) => match op_b {
                        Some(b) => {
                            let c = a - b;
                            stack.push(CalculatorInput::Value(c));
                        }
                        None => {
                            return None;
                        }
                    },
                    None => {
                        return None;
                    }
                }
            }
            CalculatorInput::Multiply => {
                let (op_a, op_b) = extract_operands(&mut stack);
                match op_a {
                    Some(a) => match op_b {
                        Some(b) => {
                            let c = a * b;
                            stack.push(CalculatorInput::Value(c));
                        }
                        None => {
                            return None;
                        }
                    },
                    None => {
                        return None;
                    }
                }
            }
            CalculatorInput::Divide => {
                let (op_a, op_b) = extract_operands(&mut stack);
                match op_a {
                    Some(a) => match op_b {
                        Some(b) => {
                            let c = a / b;
                            stack.push(CalculatorInput::Value(c));
                        }
                        None => {
                            return None;
                        }
                    },
                    None => {
                        return None;
                    }
                }
            }
            CalculatorInput::Value(v) => {
                stack.push(CalculatorInput::Value(*v));
            }
        }
    }
    if stack.len() == 1 {
        let result = match stack[0] {
            CalculatorInput::Value(v) => Some(v),
            _ => None,
        };

        result
    } else {
        None
    }
}
