use std::collections::HashMap;

type CalculationHashTree = (HashMap<i32, HashMap<char, Operator>>, HashMap<char, Operator>);

struct OperationTracker {
    x1: i32,
    x2: i32,
    calculable: Option<Operator>,
}


pub fn calc(query: String) -> String {
    let (priorites, all_operation_characters) = BuildCalculationHashTree();
    let mut result: String = String::from("");
    let mut done = false;

    let mut operationTracker = FindNextOperation(query);


    while match operationTracker {
        None => {false}
        Some(_) => {true}
    } {

        for (x, y) in &priorites {

        };

    }


    return result;
}

fn FindNextOperation(query: String) -> Option<OperationTracker> {
    for (index, char) in query.chars().enumerate() {}


    return None;
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