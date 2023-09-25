use crate::calculables::{Operator};
use crate::calculation_hash_tree::OperatorCommands;

pub fn find_next_operation(query: &String, operator_commands: &OperatorCommands) -> Option<OperationTracker> {
    let mut operator: Option<Operator> = None;
    let mut left_start_of_operation: Option<usize> = None;
    let mut left_end_of_operation: Option<usize> = None;
    let mut right_start_of_operation: Option<usize> = None;
    let mut right_end_of_operation: Option<usize> = None;

    let mut index = 0;
    let query_ref = query.as_str();

    while index <= query.len() {
        let char_option = query_ref.get(index..index + 1);

        if char_option.is_none() {
            return None;
        }

        let char = char_option.unwrap();


        if operator_commands.contains_key(char) {
            left_end_of_operation = Some(index - 1);
            right_start_of_operation = Some(index + 1);
            operator = Some(operator_commands[&char]);

            // backtrack over the query to find all numbers that need to be processed
            let mut i = index - 1;
            while i >= 0 {
                let inner_char = query_ref.get(i..i + 1).unwrap();
                let actual_char = inner_char.chars().next().unwrap();

                if !actual_char.is_numeric() && actual_char != '.' {

                    left_start_of_operation = Some(i+1);
                    break;
                }

                if i == 0 {
                    left_start_of_operation = Some(i);
                    break;
                }

                i = i - 1;
            }

            // move forward in the query to find all numbers that need to be processed
            let mut u = index + 1;
            while u < query.len() {
                let inner_char = query_ref.get(u..u + 1).unwrap();
                let actual_char = inner_char.chars().next().unwrap();

                if !actual_char.is_numeric() && actual_char != '.' {
                    right_end_of_operation = Some(u - 1);
                    u = u + 1;
                    break;
                }


                if u == query.len() - 1 {
                    right_end_of_operation = Some(u);
                    break;
                }
                u = u + 1;
            }
            break;
        }
        index = index + 1;
    }

    for x in Vec::from([left_start_of_operation.is_none(), left_end_of_operation.is_none(), right_start_of_operation.is_none(), right_end_of_operation.is_none(), operator.is_none()]) {
        if x == true {
            return None;
        }
    }

    let result = Some(OperationTracker {
        left_start: left_start_of_operation.unwrap(),
        left_end: left_end_of_operation.unwrap(),
        right_start: right_start_of_operation.unwrap(),
        right_end: right_end_of_operation.unwrap(),
        calculable: operator.unwrap(),
    });

    return result;
}

pub struct OperationTracker {
    pub left_start: usize,
    pub left_end: usize,
    pub right_start: usize,
    pub right_end: usize,
    pub calculable: Operator,
}

#[test]
fn operationIsParsedProperly() {
    let result = find_next_operation(&String::from("1+1"), &HashMap::from([("+", Addition::new())]));
    assert_eq!(result.is_none(), false);


    let operation_tracker = result.unwrap();

    assert_eq!(0, operation_tracker.left_start);
    assert_eq!(0, operation_tracker.left_end);
    assert_eq!(2, operation_tracker.right_start);
    assert_eq!(2, operation_tracker.right_end);
}

#[test]
fn queryHasNoCalculableValues() {
    let result = find_next_operation(&String::from("123123"), &HashMap::from([("+", Addition::new())]));
    assert_eq!(result.is_none(), true);
}

#[test]
fn multipleOperationsInOneExpression() {
    let result = find_next_operation(&String::from("4+5-1"), &HashMap::from([("+", Addition::new()), ("-", Subtraction::new())]));
    assert_eq!(result.is_none(), false);


    let operation_tracker = result.unwrap();

    assert_eq!(0, operation_tracker.left_start);
    assert_eq!(0, operation_tracker.left_end);
    assert_eq!(2, operation_tracker.right_start);
    assert_eq!(2, operation_tracker.right_end);
}

#[test]
fn multipleOperationsOrderForCommandsInitDoesNotMatter() {
    let result = find_next_operation(&String::from("4+5-1"), &HashMap::from([("-", Subtraction::new()), ("+", Addition::new())]));
    assert_eq!(result.is_none(), false);


    let operation_tracker = result.unwrap();

    assert_eq!(0, operation_tracker.left_start);
    assert_eq!(0, operation_tracker.left_end);
    assert_eq!(2, operation_tracker.right_start);
    assert_eq!(2, operation_tracker.right_end);
}

#[test]
fn decimalsShouldBeHandledProperly() {
    let result = find_next_operation(&String::from("0.5+0.5"), &HashMap::from([("-", Subtraction::new()), ("+", Addition::new())]));
    assert_eq!(result.is_none(), false);


    let operation_tracker = result.unwrap();

    assert_eq!(0, operation_tracker.left_start);
    assert_eq!(2, operation_tracker.left_end);
    assert_eq!(4, operation_tracker.right_start);
    assert_eq!(6, operation_tracker.right_end);
}

#[test]
fn multipleOperationsOrderForCommandsInitDoesNotMatter2() {
    let result = find_next_operation(&String::from("9-1"), &HashMap::from([("-", Subtraction::new()), ("+", Addition::new())]));
    assert_eq!(result.is_none(), false);


    let operation_tracker = result.unwrap();

    assert_eq!(0, operation_tracker.left_start);
    assert_eq!(0, operation_tracker.left_end);
    assert_eq!(2, operation_tracker.right_start);
    assert_eq!(2, operation_tracker.right_end);
}

#[test]
fn multiplePrioritiesShouldBeAllowed() {
    let result = find_next_operation(&String::from("4000*200+5-5"), &HashMap::from([("-", Subtraction::new()), ("+", Addition::new()), (("*"), Multiplication::new())]));
    assert_eq!(result.is_none(), false);


    let operation_tracker = result.unwrap();

    assert_eq!(0, operation_tracker.left_start);
    assert_eq!(3, operation_tracker.left_end);
    assert_eq!(5, operation_tracker.right_start);
    assert_eq!(7, operation_tracker.right_end);
}

#[test]
fn specificCase1() {
    let result = find_next_operation(&String::from("1+1*10"), &HashMap::from( [(("*"), Multiplication::new())]));
    assert_eq!(result.is_none(), false);


    let operation_tracker = result.unwrap();

    assert_eq!(2, operation_tracker.left_start);
    assert_eq!(2, operation_tracker.left_end);
    assert_eq!(4, operation_tracker.right_start);
    assert_eq!(5, operation_tracker.right_end);
}