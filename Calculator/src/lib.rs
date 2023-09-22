use std::collections::HashMap;

type CalculationHashTree = (HashMap<i32, OperatorCommands>, OperatorCommands);
type OperatorCommands = HashMap<char, Operator>;
struct OperationTracker {
    leftStart: usize,
    leftEnd: usize,
    rightStart:usize,
    rightEnd: usize,
    calculable: Option<Operator>,
}


pub fn calc(query: &String) -> String {
    let (priorites, all_operation_characters) = BuildCalculationHashTree();
    let mut result: String = String::from("");
    let mut done = false;

    for (priorityIndex, operatorCommands) in priorites {

        let operation = FindNextOperation(query, operatorCommands);

    }
    


    return result;
}

fn FindNextOperation(query: &String, operatorCommands: HashMap<char, Operator>) -> Option<OperationTracker> {

    let mut operator: Option<Operator> = None;
    let mut leftStartOfOperation: Option<usize> = None;
    let mut leftEndOfOperation: Option<usize> = None;
    let mut rightStartOfOperation: Option<usize> = None;
    let mut rightEndOfOperation: Option<usize> = None;


    let queryChars = query.as_bytes();
    let queryEnumerator = queryChars.iter().enumerate();

    for (index, u8char) in queryEnumerator {

        let char = char::from(*u8char);

        if(operatorCommands.contains_key(&char)) {
            leftEndOfOperation = Option::from(index - 1);
            rightStartOfOperation = Option::from(index + 1);
            operator = Option::from(operatorCommands[&char]);

            // backtrack over the query to find all numbers that need to be processed
            let mut i = index;
            while i >= 0 {
                let innerChar: char = char::from(queryChars[i]);

                if(!innerChar.is_numeric()) {
                    leftStartOfOperation = Option::from(i+1);
                    break;
                }


                if(i == 0) {
                    leftStartOfOperation = Option::from(i);
                    break;
                }
                i = i-1;
            }

            // move forward in the query to find all numbers that need to be processed
            let mut i = index;
            while i <= 0 {
                let innerChar: char = char::from(queryChars[i]);

                if(!innerChar.is_numeric()) {
                    rightEndOfOperation = Option::from(i-1);
                    break;
                }


                if(i == query.len()) {
                    rightEndOfOperation = Option::from(i);
                    break;
                }
                i = i+1;
            }
        }
    }

    if(leftStartOfOperation == None) {
        return None;
    }

    if(leftEndOfOperation == None) {
        return None;
    }

    if(rightStartOfOperation == None) {
        return None;
    }

    if(rightEndOfOperation == None) {
        return None;
    }


    return Option::from(OperationTracker {
        leftStart: leftStartOfOperation.unwrap(),
        leftEnd: leftEndOfOperation.unwrap(),
        rightStart: rightStartOfOperation.unwrap(),
        rightEnd: rightEndOfOperation.unwrap(),
        calculable: operator,
    });
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


pub type Calculable = fn(values: Vec<i32>, query: Option<String>) -> String;

#[derive(Clone, Copy)]
struct Operator {
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
            calculable: |values, query| {
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
            calculable: |values, query| {
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
            calculable: |values, query| {
                return (values.get(0).unwrap() / values.get(1).unwrap()).to_string();
            },
        }
    }
}