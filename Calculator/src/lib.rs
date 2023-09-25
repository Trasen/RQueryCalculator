use std::collections::{BTreeMap, HashMap};

mod CalculationTracker;

type CalculationHashTree = (BTreeMap<i32, OperatorCommands>, OperatorCommands);
type OperatorCommands = HashMap<&'static str, Operator>;

pub fn calc(query: &String) -> String {
    let (priorites, _all_operation_characters) = BuildCalculationHashTree();

    let mut result: String = String::from(str::replace(query.as_str(), " ", "")).trim().parse().unwrap();

    let mut groupsDone = false;

    let mut i = 0;

    let mut openBrace:Option<usize> = None;
    let mut closedBrace: Option<usize> = None;

    while !groupsDone {

        let queryRef = result.as_str();

        let actualChar = queryRef.get(i..i + 1).unwrap().chars().next().unwrap();

        if actualChar == '(' {
            openBrace = Some(i);
            closedBrace = None;
        }

        if actualChar == ')' {
            closedBrace = Some(i);
        }

        if(!openBrace.is_none() && !closedBrace.is_none()) {

            let mut calculable = String::from(result.get(openBrace.unwrap() + 1 .. closedBrace.unwrap()).unwrap());;

            for (_priorityIndex, operatorCommands) in &priorites {

                let mut priorityDone = false;

                while !priorityDone {
                    let operation = CalculationTracker::FindNextOperation(&calculable, &operatorCommands);

                    match operation {
                        Some(operationTracker) => {
                            let leftRange = operationTracker.leftStart..operationTracker.leftEnd + 1;
                            let rightRange = operationTracker.rightStart..operationTracker.rightEnd + 1;

                            let leftNumbers = &calculable.get(leftRange).unwrap().parse::<LargeDecimal>().unwrap();
                            let rightNumbers = &calculable.get(rightRange).unwrap().parse::<LargeDecimal>().unwrap();

                            let finishedCalculation = (operationTracker.calculable.calculable)(Vec::from([*leftNumbers, *rightNumbers]), Option::from(&calculable));

                            calculable.replace_range((operationTracker.leftStart ..operationTracker.rightEnd + 1), finishedCalculation.as_str());
                        }
                        None => { priorityDone = true }
                    }
                }
            }

            result.replace_range(openBrace.unwrap() ..closedBrace.unwrap() + 1, &calculable);
            openBrace = None;
            closedBrace = None;
            i = 0;
            continue;
        }

        i = i + 1;

        if(i >= queryRef.len()) {
            groupsDone = true;
        }

    }

    for (_priorityIndex, operatorCommands) in priorites {

        let mut priorityDone = false;

        while !priorityDone {
            let operation = CalculationTracker::FindNextOperation(&result, &operatorCommands);

            match operation {
                Some(operationTracker) => {
                    let leftRange = operationTracker.leftStart..operationTracker.leftEnd + 1;
                    let rightRange = operationTracker.rightStart..operationTracker.rightEnd + 1;

                    let leftNumbers = &result.get(leftRange).unwrap().parse::<LargeDecimal>().unwrap();
                    let rightNumbers = &result.get(rightRange).unwrap().parse::<LargeDecimal>().unwrap();

                    let finishedCalculation = (operationTracker.calculable.calculable)(Vec::from([*leftNumbers, *rightNumbers]), Option::from(&result));

                    result.replace_range((operationTracker.leftStart..operationTracker.rightEnd + 1), finishedCalculation.as_str());

                }
                None => { priorityDone = true }
            }
        }
    }

    return result;
}

fn BuildCalculationHashTree() -> CalculationHashTree {
    let mut priorites: BTreeMap<i32, HashMap<&str, Operator>> = BTreeMap::new();
    let mut operatorCharacters = HashMap::new();

    let commands: Vec<Operator> = Vec::from([Addition::new(), Subtraction::new(), Multiplication::new(), Division::new()]);

    for x in commands {
        let prio: i32 = x.priority;
        let priority = priorites.get_mut(&prio);
        let char: &str = x.char;

        operatorCharacters.insert(char, x);

        let mut map = HashMap::new();
        map.insert(char, x);

        match priority {
            None => {
                priorites.insert(prio, map);
            }
            Some(some) => {
                some.insert(char, x);
            }
        }
    }
    (priorites, operatorCharacters)
}


pub type LargeDecimal = f64;
pub type Calculable = fn(values: Vec<LargeDecimal>, query: Option<&String>) -> String;

#[derive(Clone, Copy)]
pub struct Operator {
    priority: i32,
    char: &'static str,
    pub calculable: Calculable,
}

struct Addition {}

struct Subtraction {}

struct Multiplication {}

struct Division {}


impl Addition {
    fn new() -> Operator {
        Operator {
            priority: 4,
            char: "+",
            calculable: |values, _query| {
                return (values.get(0).unwrap() + values.get(1).unwrap()).to_string();
            },
        }
    }
}

impl Subtraction {
    fn new() -> Operator {
        Operator {
            priority: 4,
            char: "-",
            calculable: |values, query| {
                return (values.get(0).unwrap() - values.get(1).unwrap()).to_string();
            },
        }
    }
}

impl Multiplication {
    fn new() -> Operator {
        Operator {
            priority: 2,
            char: "*",
            calculable: |values, _query| {
                return (values.get(0).unwrap() * values.get(1).unwrap()).to_string();
            },
        }
    }
}

impl Division {
    fn new() -> Operator {
        Operator {
            priority: 2,
            char: "/",
            calculable: |values, _query| {
                let val1 = values.get(0).unwrap();
                let val2 = values.get(1).unwrap();

                if val1.eq(&0.0) || val2.eq(&0.0)  {
                    return 0.to_string();
                }

                return (values.get(0).unwrap() / values.get(1).unwrap()).to_string();
            },
        }
    }
}
