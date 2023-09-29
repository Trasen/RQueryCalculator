use std::collections::HashMap;

use crate::calculables::{Addition, Multiplication, Operator, Subtraction};
use crate::calculation_hash_tree::OperatorCommands;

pub fn find_next_operation(query: &String, operator_commands: &OperatorCommands) -> Option<OperationTracker> {

    let mut operator: Option<Operator> = None;
    let mut left_start_of_operation: Option<usize> = None;
    let mut left_end_of_operation: Option<usize> = None;
    let mut right_start_of_operation: Option<usize> = None;
    let mut right_end_of_operation: Option<usize> = None;

    let mut index = 0;

    while index <= query.len() {
        let char_option = match query.get(index..index + 1) {
            None => {""}
            Some(char) => {char}
        };

        let char = char_option;

        if(index == 0 && char == "-") { // first value is negative, not subtraction
            index = index + 1;
            continue;
        }

        if operator_commands.contains_key(char) {

            left_end_of_operation = Some(index - 1);
            right_start_of_operation = Some(index + 1);
            operator = Some(operator_commands[&char]);

            // backtrack over the query to find all numbers that need to be processed
            left_start_of_operation = backtrack_to_find_numbers(query, &index);

            // move forward in the query to find all numbers that need to be processed
            right_end_of_operation = move_forward_to_find_numbers(query, &index);

            return Some(OperationTracker {
                left_start: left_start_of_operation.unwrap(),
                left_end: left_end_of_operation.unwrap(),
                right_start: right_start_of_operation.unwrap(),
                right_end: right_end_of_operation.unwrap(),
                calculable: operator.unwrap(),
            });
        }
        index = index + 1;
    }

    return None;
}

fn move_forward_to_find_numbers(query: &String, index: &usize) -> Option<usize> {
    let mut i = index + 1;
    while i < query.len() {
        let current_char = query.get(i..i + 1)?.chars().next().unwrap();
        let previous_char = query.get(i - 1..i + 1)?.chars().next().unwrap();

        if (!previous_char.is_numeric() && current_char == '-') {
            i = i + 1;
            continue;
        }

        if !current_char.is_numeric() && current_char != '.' {
            return Some(i - 1);
        }


        if i == query.len() - 1 {
            return Some(i);
        }
        i = i + 1;
    }
    return None;
}

fn backtrack_to_find_numbers(query: &String, index: &usize) -> Option<usize> {
    let mut i = index - 1;
    while i >= 0 {
        let current_char = query.get(i..i + 1)?.chars().next().unwrap();

        if i == 0 {
            return Some(i);
        }

        if !current_char.is_numeric() && current_char != '.' {
            return Some(i + 1);
        }

        if i == 0 {
            return Some(i);
        }

        i = i - 1;
    }
    return None;
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
fn negative_values_should_be_dealt_with_properly() {
    let result = find_next_operation(&String::from("-5*-5"), &HashMap::from([("-", Subtraction::new()), ("+", Addition::new()), (("*"), Multiplication::new())]));
    assert_eq!(result.is_none(), false);


    let operation_tracker = result.unwrap();

    assert_eq!(0, operation_tracker.left_start);
    assert_eq!(1, operation_tracker.left_end);
    assert_eq!(3, operation_tracker.right_start);
    assert_eq!(4, operation_tracker.right_end);
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