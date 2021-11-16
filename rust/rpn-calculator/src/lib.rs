#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}
pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let v: Vec<i32> = vec![];
    let mut result = inputs.iter().fold(v, |mut stack, input| {
        if let Some(new) = match input {
            CalculatorInput::Add => stack
                .pop()
                .and_then(|b| stack.pop().and_then(|a| Some(a + b))),
            CalculatorInput::Subtract => stack
                .pop()
                .and_then(|b| stack.pop().and_then(|a| Some(a - b))),
            CalculatorInput::Multiply => stack
                .pop()
                .and_then(|b| stack.pop().and_then(|a| Some(a * b))),
            CalculatorInput::Divide => stack
                .pop()
                .and_then(|b| stack.pop().and_then(|a| Some(a / b))),
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
