use std::collections::{BTreeMap, HashMap};

use crate::calculables::{Addition, Division, Multiplication, Operator, PowerOf, SquareRoot, Subtraction};

pub type CalculationHashTree = (BTreeMap<i32, OperatorCommands>, OperatorCommands);
pub type OperatorCommands = HashMap<&'static char, Operator>;
pub type OperatorPriorities = BTreeMap<i32, OperatorCommands>;

pub fn build_calculation_hash_tree() -> CalculationHashTree {
    let mut priorites: OperatorPriorities = BTreeMap::new();
    let mut operator_characters = HashMap::new();

    let commands: Vec<Operator> = Vec::from([Addition::new(), Subtraction::new(), Multiplication::new(), Division::new(), PowerOf::new(), SquareRoot::new()]);

    for x in commands {
        let prio: i32 = x.priority;
        let priority = priorites.get_mut(&prio);
        let char: &char = x.char;

        operator_characters.insert(char, x);

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
    (priorites, operator_characters)
}
