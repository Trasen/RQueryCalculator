use crate::Calculables::LargeDecimal;
use crate::CalculationTracker;
use crate::CalculationHashTree::OperatorPriorities;

pub fn resolveCalculationFromTo(priorites: &OperatorPriorities, calculableValue: &String, from: usize, to: usize) -> String {
    let calculable = String::from(calculableValue.get(from + 1..to).unwrap());

    let result = resolveCalculation(priorites, &calculable);
    return result;
}

pub fn resolveCalculation(priorites: &OperatorPriorities, calculableValue: &String) -> String {
    let mut calculable = String::from(calculableValue);

    for (_priorityIndex, operatorCommands) in priorites {
        let mut priorityDone = false;

        while !priorityDone {
            let operation = CalculationTracker::FindNextOperation(&calculable, &operatorCommands);

            match operation {
                Some(operationTracker) => {
                    let leftRange = operationTracker.leftStart..operationTracker.leftEnd + 1;
                    let rightRange = operationTracker.rightStart..operationTracker.rightEnd + 1;

                    let leftNumbers = &calculable.get(leftRange).unwrap().parse::<LargeDecimal>().unwrap();
                    let rightNumbers = &calculable.get(rightRange).unwrap().parse::<LargeDecimal>().unwrap();

                    let finishedCalculation = (operationTracker.calculable.calculable)(Vec::from([*leftNumbers, *rightNumbers]));

                    calculable.replace_range((operationTracker.leftStart..operationTracker.rightEnd + 1), finishedCalculation.as_str());
                }
                None => { priorityDone = true }
            }
        }
    }
    return calculable;
}
