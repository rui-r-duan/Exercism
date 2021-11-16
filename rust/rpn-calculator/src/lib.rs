#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];
    for input in inputs {
        match input {
            CalculatorInput::Add => {
                let b = stack.pop()?;
                let a = stack.pop()?;
                stack.push(a + b);
            }
            CalculatorInput::Subtract => {
                let b = stack.pop()?;
                let a = stack.pop()?;
                stack.push(a - b);
            }
            CalculatorInput::Multiply => {
                let b = stack.pop()?;
                let a = stack.pop()?;
                stack.push(a * b);
            }
            CalculatorInput::Divide => {
                let b = stack.pop()?;
                let a = stack.pop()?;
                stack.push(a / b);
            }
            CalculatorInput::Value(val) => {
                stack.push(*val);
            }
        }
    }

    if stack.len() > 1 {
        return None;
    }

    stack.pop()
}
