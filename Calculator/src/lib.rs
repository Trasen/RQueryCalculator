use std::collections::HashMap;

mod CalculationTracker;

type CalculationHashTree = (HashMap<i32, OperatorCommands>, OperatorCommands);
type OperatorCommands = HashMap<char, Operator>;

pub fn calc(query: &String) -> String {
    let (priorites, _all_operation_characters) = BuildCalculationHashTree();

    let mut result: String = String::from(query);

    for (_priorityIndex, operatorCommands) in priorites {

        let test = &operatorCommands;

        let mut priorityDone = false;

        while !priorityDone {

            let operation = CalculationTracker::FindNextOperation(&result, test);

            match operation {
                Some(operationTracker) => {

                    let leftRange = (operationTracker.leftStart .. operationTracker.leftEnd + 1);
                    let rightRange = (operationTracker.rightStart .. operationTracker.rightEnd + 1);

                    let leftNumbers = &result.get(leftRange).unwrap().parse::<i32>().unwrap();
                    let rightNumbers = &result.get(rightRange).unwrap().parse::<i32>().unwrap();

                    let finishedCalculation = (operationTracker.calculable.calculable)(Vec::from([*leftNumbers, *rightNumbers]), Option::from(query));

                    result.replace_range((operationTracker.leftStart .. operationTracker.rightEnd + 1), finishedCalculation.as_str());


                }
                None => { priorityDone = true}
            }
        }

    }

    return result;
}

fn BuildCalculationHashTree() -> CalculationHashTree {
    let mut priorites: HashMap<i32, HashMap<char, Operator>> = HashMap::new();
    let mut operatorCharacters = HashMap::new();

    let commands: Vec<Operator> = Vec::from([Addition::new(), Subtraction::new(), Multiplication::new(), Division::new()]);

    for x in commands {
        let prio: i32 = x.priority;
        let priority = priorites.get_mut(&prio);
        let char: char = x.char;

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


pub type Calculable = fn(values: Vec<i32>, query: Option<&String>) -> String;

#[derive(Clone, Copy)]
pub struct Operator {
    priority: i32,
    char: char,
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
            char: '+',
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
            char: '-',
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
            char: '*',
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
            char: '/',
            calculable: |values, _query| {
                return (values.get(0).unwrap() / values.get(1).unwrap()).to_string();
            },
        }
    }
}
