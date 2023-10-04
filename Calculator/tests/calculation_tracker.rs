use RQueryCalculator::{calculation_tracker};
use RQueryCalculator::calculables::{Addition, Multiplication, SquareRoot, Subtraction};
use RQueryCalculator::calculation_hash_tree::{OperatorCommands};

#[test]
fn operationIsParsedProperly() {
    let result = calculation_tracker::find_next_operation(&String::from("1+1").chars().collect(), &OperatorCommands::from([(&'+', Addition::new())]));
    assert_eq!(result.is_none(), false);


    let operation_tracker = result.unwrap();

    assert_eq!(0, operation_tracker.left_start);
    assert_eq!(0, operation_tracker.left_end);
    assert_eq!(2, operation_tracker.right_start);
    assert_eq!(2, operation_tracker.right_end);
}

#[test]
fn queryHasNoCalculableValues() {
    let result = calculation_tracker::find_next_operation(&String::from("123123").chars().collect(), &OperatorCommands::from([(&'+', Addition::new())]));
    assert_eq!(result.is_none(), true);
}

#[test]
fn multipleOperationsInOneExpression() {
    let result = calculation_tracker::find_next_operation(&String::from("4+5-1").chars().collect(), &OperatorCommands::from([(&'+', Addition::new()), (&'-', Subtraction::new())]));
    assert_eq!(result.is_none(), false);


    let operation_tracker = result.unwrap();

    assert_eq!(0, operation_tracker.left_start);
    assert_eq!(0, operation_tracker.left_end);
    assert_eq!(2, operation_tracker.right_start);
    assert_eq!(2, operation_tracker.right_end);
}

#[test]
fn multipleOperationsOrderForCommandsInitDoesNotMatter() {
    let result = calculation_tracker::find_next_operation(&String::from("4+5-1").chars().collect(), &OperatorCommands::from([(&'-', Subtraction::new()), (&'+', Addition::new())]));
    assert_eq!(result.is_none(), false);


    let operation_tracker = result.unwrap();

    assert_eq!(0, operation_tracker.left_start);
    assert_eq!(0, operation_tracker.left_end);
    assert_eq!(2, operation_tracker.right_start);
    assert_eq!(2, operation_tracker.right_end);
}

#[test]
fn decimalsShouldBeHandledProperly() {
    let result = calculation_tracker::find_next_operation(&String::from("0.5+0.5").chars().collect(), &OperatorCommands::from([(&'-', Subtraction::new()), (&'+', Addition::new())]));
    assert_eq!(result.is_none(), false);


    let operation_tracker = result.unwrap();

    assert_eq!(0, operation_tracker.left_start);
    assert_eq!(2, operation_tracker.left_end);
    assert_eq!(4, operation_tracker.right_start);
    assert_eq!(6, operation_tracker.right_end);
}

#[test]
fn multipleOperationsOrderForCommandsInitDoesNotMatter2() {
    let result = calculation_tracker::find_next_operation(&String::from("9-1").chars().collect(), &OperatorCommands::from([(&'-', Subtraction::new()), (&'+', Addition::new())]));
    assert_eq!(result.is_none(), false);


    let operation_tracker = result.unwrap();

    assert_eq!(0, operation_tracker.left_start);
    assert_eq!(0, operation_tracker.left_end);
    assert_eq!(2, operation_tracker.right_start);
    assert_eq!(2, operation_tracker.right_end);
}

#[test]
fn multiplePrioritiesShouldBeAllowed() {
    let result = calculation_tracker::find_next_operation(&String::from("4000*200+5-5").chars().collect(), &OperatorCommands::from([(&'-', Subtraction::new()), (&'+', Addition::new()), (&('*'), Multiplication::new())]));
    assert_eq!(result.is_none(), false);


    let operation_tracker = result.unwrap();

    assert_eq!(0, operation_tracker.left_start);
    assert_eq!(3, operation_tracker.left_end);
    assert_eq!(5, operation_tracker.right_start);
    assert_eq!(7, operation_tracker.right_end);
}

#[test]
fn negative_values_should_be_dealt_with_properly() {
    let result = calculation_tracker::find_next_operation(&String::from("-5*-5").chars().collect(), &OperatorCommands::from([(&'-', Subtraction::new()), (&'+', Addition::new()), (&('*'), Multiplication::new())]));
    assert_eq!(result.is_none(), false);


    let operation_tracker = result.unwrap();

    assert_eq!(0, operation_tracker.left_start);
    assert_eq!(1, operation_tracker.left_end);
    assert_eq!(3, operation_tracker.right_start);
    assert_eq!(4, operation_tracker.right_end);
}

#[test]
fn specificCase1() {
    let result = calculation_tracker::find_next_operation(&String::from("1+1*10").chars().collect(), &OperatorCommands::from( [(&('*'), Multiplication::new())]));
    assert_eq!(result.is_none(), false);


    let operation_tracker = result.unwrap();

    assert_eq!(2, operation_tracker.left_start);
    assert_eq!(2, operation_tracker.left_end);
    assert_eq!(4, operation_tracker.right_start);
    assert_eq!(5, operation_tracker.right_end);
}

#[test]
fn functions_should_work_as_intended() {
    let result = calculation_tracker::find_next_operation(&String::from("√4").chars().collect(), &OperatorCommands::from( [(&('√'), SquareRoot::new())]));
    assert_eq!(result.is_none(), false);

    let operation_tracker = result.unwrap();

    assert_eq!(1, operation_tracker.right_start);
    assert_eq!(1, operation_tracker.right_end);
}

#[test]
fn special_case() {
    let result = calculation_tracker::find_next_operation(&String::from("asdasd -600 asdasd" ).chars().collect(), &OperatorCommands::from( [(&('√'), SquareRoot::new())]));
    assert_eq!(result.is_none(), true);
}
