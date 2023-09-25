mod CalculationTracker;
mod CalculationResolver;
mod Calculables;
mod CalculationHashTree;


pub fn calc(query: String) -> String {
    let (priorites, _all_operation_characters) = CalculationHashTree::BuildCalculationHashTree();

    let mut result: String = String::from(str::replace(query.as_str(), " ", "")).trim().parse().unwrap();

    let mut groupsDone = false;

    let mut i = 0;

    let mut openBrace: Option<usize> = None;
    let mut closedBrace: Option<usize> = None;

    while !groupsDone {
        let currentChar = result.get(i..i + 1).unwrap().chars().next().unwrap();

        if currentChar == '(' {
            openBrace = Some(i);
            closedBrace = None;
        }

        if currentChar == ')' {
            closedBrace = Some(i);
        }

        if !openBrace.is_none() && !closedBrace.is_none() {
            let calculable = CalculationResolver::resolveCalculationFromTo(&priorites, &result, openBrace.unwrap(), closedBrace.unwrap());

            result.replace_range(openBrace.unwrap()..closedBrace.unwrap() + 1, &calculable);
            openBrace = None;
            closedBrace = None;
            i = 0;
            continue;
        }

        i = i + 1;

        if (i >= result.len()) {
            groupsDone = true;
        }
    }

    result = CalculationResolver::resolveCalculation(&priorites, &result);

    return result;
}

