#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<CalculatorInput> = vec![];

    for input in inputs {
        handle_input(&mut stack, input)?;
    }

    get_result_or_none_from_stack(&stack)
}

fn get_result_or_none_from_stack(stack: &Vec<CalculatorInput>) -> Option<i32> {
    if let [CalculatorInput::Value(result)] = &stack[..] {
        Some(*result)
    } else {
        None
    }
}

fn handle_input(stack: &mut Vec<CalculatorInput>, input: &CalculatorInput) -> Option<()> {
    match input {
        CalculatorInput::Value(n) => stack.push(CalculatorInput::Value(*n)),
        operator @ _ => {
            let (first, second) = pop_two_elements_from_stack(stack)?;
            let value = CalculatorInput::Value(calculate(first, second, operator));
            stack.push(value);
        }
    }

    Some(())
}

fn calculate(a: i32, b: i32, operator: &CalculatorInput) -> i32 {
    match operator {
        CalculatorInput::Add => a + b,
        CalculatorInput::Subtract => a - b,
        CalculatorInput::Multiply => a * b,
        CalculatorInput::Divide => a / b,
        _ => panic!("How did you get here?"),
    }
}

fn pop_two_elements_from_stack(vec: &mut Vec<CalculatorInput>) -> Option<(i32, i32)> {
    let a = pop_value_from_stack(vec)?;
    let b = pop_value_from_stack(vec)?;

    Some((b, a))
}

fn pop_value_from_stack(vec: &mut Vec<CalculatorInput>) -> Option<i32> {
    if let CalculatorInput::Value(v) = vec.pop()? {
        Some(v)
    } else {
        None
    }
}

#[cfg(test)]
fn calculator_input(s: &str) -> Vec<CalculatorInput> {
    s.split_whitespace()
        .map(|s| match s {
            "+" => CalculatorInput::Add,
            "-" => CalculatorInput::Subtract,
            "*" => CalculatorInput::Multiply,
            "/" => CalculatorInput::Divide,
            n => CalculatorInput::Value(n.parse().unwrap()),
        })
        .collect()
}

#[test]
fn test_empty_input_returns_none() {
    let input = calculator_input("");
    assert_eq!(evaluate(&input), None);
}

#[test]
fn test_simple_value() {
    let input = calculator_input("10");
    assert_eq!(evaluate(&input), Some(10));
}

#[test]
fn test_simple_addition() {
    let input = calculator_input("2 2 +");
    assert_eq!(evaluate(&input), Some(4));
}

#[test]
fn test_simple_subtraction() {
    let input = calculator_input("7 11 -");
    assert_eq!(evaluate(&input), Some(-4));
}

#[test]
fn test_simple_multiplication() {
    let input = calculator_input("6 9 *");
    assert_eq!(evaluate(&input), Some(54));
}

#[test]
fn test_simple_division() {
    let input = calculator_input("57 19 /");
    assert_eq!(evaluate(&input), Some(3));
}

#[test]
fn test_complex_operation() {
    let input = calculator_input("4 8 + 7 5 - /");
    assert_eq!(evaluate(&input), Some(6));
}

#[test]
fn test_too_few_operands_returns_none() {
    let input = calculator_input("2 +");
    assert_eq!(evaluate(&input), None);
}

#[test]
fn test_too_many_operands_returns_none() {
    let input = calculator_input("2 2");
    assert_eq!(evaluate(&input), None);
}

#[test]
fn test_zero_operands_returns_none() {
    let input = calculator_input("+");
    assert_eq!(evaluate(&input), None);
}

#[test]
fn test_intermediate_error_returns_none() {
    let input = calculator_input("+ 2 2 *");
    assert_eq!(evaluate(&input), None);
}
