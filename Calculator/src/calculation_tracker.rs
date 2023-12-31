use crate::calculables::{Operator, OperatorType};
use crate::calculation_hash_tree::OperatorCommands;

pub fn find_next_operation(query: &Vec<char>, operator_commands: &OperatorCommands) -> Option<OperationTracker> {

    let mut index = 0;
    let mut previous_char: Option<char> = None;
    while index < query.len() {
        let current_char = query.get(index).unwrap();


        // Deal with Negative Number is not an Operator (again)
        match previous_char {
            None => {
                if current_char == &'-' {
                    index = index + 1;
                    continue;
                }
            }
            Some(some) => {
                if current_char == &'-' && !some.is_numeric() {
                    index = index + 1;
                    continue;
                }
            }
        }


        previous_char = Some(*current_char);

        if operator_commands.contains_key(&current_char) {
            let operator = operator_commands[&current_char];

            match operator.operator_type {
                OperatorType::Operator => {
                    let mut left_start_of_operation: Option<usize> = None;
                    let mut left_end_of_operation: Option<usize> = Some(index - 1);
                    let mut right_start_of_operation: Option<usize> = Some(index + 1);
                    let mut right_end_of_operation: Option<usize> = None;

                    // backtrack over the query to find all numbers that need to be processed
                    left_start_of_operation = backtrack_to_find_numbers(&query, &index);

                    // move forward in the query to find all numbers that need to be processed
                    right_end_of_operation = move_forward_to_find_numbers(&query, &index);

                    if left_start_of_operation == None {
                        return None;
                    }

                    if right_end_of_operation == None {
                        return None;
                    }

                    return Some(OperationTracker {
                        left_start: left_start_of_operation.unwrap(),
                        left_end: left_end_of_operation.unwrap(),
                        right_start: right_start_of_operation.unwrap(),
                        right_end: right_end_of_operation.unwrap(),
                        calculable: operator,
                    });
                }
                OperatorType::Function => {
                    let mut right_start_of_operation: Option<usize> = Some(index + 1);
                    let mut right_end_of_operation: Option<usize> = None;

                    // move forward in the query to find all numbers that need to be processed
                    right_end_of_operation = move_forward_to_find_numbers(&query, &index);

                    if right_end_of_operation == None {
                        return None;
                    }

                    return Some(OperationTracker {
                        left_start: 0,
                        left_end: 0,
                        right_start: right_start_of_operation.unwrap(),
                        right_end: right_end_of_operation.unwrap(),
                        calculable: operator,
                    });
                }
            }


        }
        index = index + 1;
    }

    return None;
}

fn move_forward_to_find_numbers(query: &Vec<char>, index: &usize) -> Option<usize> {
    let mut i = index + 1;
    while i < query.len() {
        let current_char = query.get(i).unwrap();
        let previous_char = query.get(i-1).unwrap();

        if (!previous_char.is_numeric() && current_char == &'-') {
            i = i + 1;
            continue;
        }

        if !current_char.is_numeric() && current_char != &'.' {
            return Some(i - 1);
        }


        if i == query.len() - 1 {
            return Some(i);
        }
        i = i + 1;
    }
    return None;
}

fn backtrack_to_find_numbers(query: &Vec<char>, index: &usize) -> Option<usize> {
    let mut i = index - 1;
    while i >= 0 {
        let current_char = query.get(i).unwrap();

        if i == 0 {
            return Some(i);
        }

        if !current_char.is_numeric() && current_char != &'.' {
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