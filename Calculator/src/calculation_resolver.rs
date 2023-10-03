use crate::calculables::LargeDecimal;
use crate::calculation_hash_tree::OperatorPriorities;
use crate::calculation_tracker;

pub fn resolve_calculation_from_to<'a>(priorites: &OperatorPriorities, calculable_value: &Vec<char>, from: usize, to: usize) -> Vec<char> {
    let calculable: Vec<char> = calculable_value[from..to].to_owned();

    let result = resolve_calculation(priorites, calculable);
    return result;
}

pub fn resolve_calculation<'a>(priorites: &OperatorPriorities, calculable_value: Vec<char>) -> Vec<char> {
    let mut calculable = calculable_value.to_owned();

    for (_, operator_commands) in priorites {
        let mut priority_done = false;

        while !priority_done {
            let operation = calculation_tracker::find_next_operation(&calculable, &operator_commands);

            match operation {
                Some(operation_tracker) => {
                    let left_range = operation_tracker.left_start..operation_tracker.left_end + 1;
                    let right_range = operation_tracker.right_start..operation_tracker.right_end + 1;

                    let left_numbers_string = &calculable.get(left_range).unwrap().into_iter().collect::<String>();
                    let right_numbers_string = &calculable.get(right_range).unwrap().into_iter().collect::<String>();

                    let left_numbers = &left_numbers_string.parse::<LargeDecimal>().unwrap();
                    let right_numbers = &right_numbers_string.parse::<LargeDecimal>().unwrap();

                    let finished_calculation = (operation_tracker.calculable.calculable)(Vec::from([*left_numbers, *right_numbers]));

                    Vec::splice(&mut calculable, operation_tracker.left_start..operation_tracker.right_end + 1, finished_calculation);
                }
                None => { priority_done = true }
            }
        }
    }
    return calculable;
}
