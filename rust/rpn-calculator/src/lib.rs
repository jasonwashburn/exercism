#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    let mut value: i32;
    for input in inputs {
        match input {
            CalculatorInput::Value(x) => stack.push(*x),
            op => {
                if stack.len() >= 2 {
                    let value2 = stack.pop().unwrap();
                    let value1 = stack.pop().unwrap();
                    value = match op {
                        CalculatorInput::Add => value1 + value2,
                        CalculatorInput::Subtract => value1 - value2,
                        CalculatorInput::Multiply => value1 * value2,
                        CalculatorInput::Divide => value1 / value2,
                        _ => return None,
                    };
                    stack.push(value);
                } else {
                    return None;
                }
            }
        }
    }
    if stack.len() == 1 {
        return Some(stack.pop().unwrap());
    } else {
        return None;
    }
}
