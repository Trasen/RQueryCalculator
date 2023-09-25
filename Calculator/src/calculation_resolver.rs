use crate::calculables::LargeDecimal;
use crate::calculation_hash_tree::OperatorPriorities;
use crate::calculation_tracker;

pub fn resolve_calculation_from_to(priorites: &OperatorPriorities, calculable_value: &String, from: usize, to: usize) -> String {
    let calculable = String::from(calculable_value.get(from + 1..to).unwrap());

    let result = resolve_calculation(priorites, &calculable);
    return result;
}

pub fn resolve_calculation(priorites: &OperatorPriorities, calculable_value: &String) -> String {
    let mut calculable = String::from(calculable_value);

    for (_, operator_commands) in priorites {
        let mut priority_done = false;

        while !priority_done {
            let operation = calculation_tracker::find_next_operation(&calculable, &operator_commands);

            match operation {
                Some(operation_tracker) => {
                    let left_range = operation_tracker.left_start..operation_tracker.left_end + 1;
                    let right_range = operation_tracker.right_start..operation_tracker.right_end + 1;

                    let left_numbers = &calculable.get(left_range).unwrap().parse::<LargeDecimal>().unwrap();
                    let right_numbers = &calculable.get(right_range).unwrap().parse::<LargeDecimal>().unwrap();

                    let finished_calculation = (operation_tracker.calculable.calculable)(Vec::from([*left_numbers, *right_numbers]));

                    calculable.replace_range(operation_tracker.left_start..operation_tracker.right_end + 1, finished_calculation.as_str());
                }
                None => { priority_done = true }
            }
        }
    }
    return calculable;
}
