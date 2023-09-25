use crate::calculation_hash_tree::CalculationHashTree;
use crate::calculation_resolver;

pub fn calculate(query: String, calculation_hash_tree: CalculationHashTree) -> String {
    let (priorities, _all_operation_characters) = calculation_hash_tree;

    let mut result: String = String::from(str::replace(query.as_str(), " ", "")).trim().parse().unwrap();

    let mut groups_done = false;

    let mut i = 0;

    let mut open_brace: Option<usize> = None;
    let mut closed_brace: Option<usize> = None;

    while !groups_done {
        let current_char = result.get(i..i + 1).unwrap().chars().next().unwrap();

        if current_char == '(' {
            open_brace = Some(i);
            closed_brace = None;
        }

        if current_char == ')' {
            closed_brace = Some(i);
        }

        if !open_brace.is_none() && !closed_brace.is_none() {
            let calculable = calculation_resolver::resolve_calculation_from_to(&priorities, &result, open_brace.unwrap(), closed_brace.unwrap());

            result.replace_range(open_brace.unwrap()..closed_brace.unwrap() + 1, &calculable);
            open_brace = None;
            closed_brace = None;
            i = 0;
            continue;
        }

        i = i + 1;

        if i >= result.len() {
            groups_done = true;
        }
    }

    result = calculation_resolver::resolve_calculation(&priorities, &result);

    return result;
}
