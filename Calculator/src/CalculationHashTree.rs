use std::collections::{BTreeMap, HashMap};
use crate::Calculables::{Addition, Division, Multiplication, Operator, Subtraction};

pub type CalculationHashTree = (BTreeMap<i32, OperatorCommands>, OperatorCommands);
pub type OperatorCommands = HashMap<&'static str, Operator>;

pub type OperatorPriorities = BTreeMap<i32, OperatorCommands>;

pub fn BuildCalculationHashTree() -> CalculationHashTree {
    let mut priorites: OperatorPriorities = BTreeMap::new();
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
