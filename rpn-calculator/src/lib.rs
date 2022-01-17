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
    
    for input in inputs.iter() {
        match input {
            CalculatorInput::Value(x) => stack.push(*x),
            CalculatorInput::Add => {
                let result = validate_operation(&mut stack);
                
                match result {
                    Ok((a, b)) => stack.push(a + b),
                    Err(_) => return None,
                }
            },
            CalculatorInput::Subtract => {
                let result = validate_operation(&mut stack);
                
                match result {
                    Ok((a, b)) => stack.push(a - b),
                    Err(_) => return None,
                }
            },
            CalculatorInput::Multiply => {
                 let result = validate_operation(&mut stack);
                
                match result {
                    Ok((a, b)) => stack.push(a * b),
                    Err(_) => return None,
                }
            },
            CalculatorInput::Divide => {
                let result = validate_operation(&mut stack);
                
                match result {
                    Ok((a, b)) => stack.push(a / b),
                    Err(_) => return None,
                }
            },
        }
    }

    if stack.len() == 1 {
        return Some(stack[0])
    } else {
        return None;
    } 
}

fn validate_operation(stack: &mut Vec<i32>) -> Result<(i32, i32), String> {
    if stack.len() >= 2 {
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();

        return Ok((a, b));
    } else {
        return Err(String::from("error"));
    }
}
