pub type Calculable = fn(values: Vec<LargeDecimal>) -> Vec<char>;

pub struct Addition {}
pub struct Subtraction {}
pub struct Multiplication {}
pub struct Division {}
pub struct PowerOf {}

pub struct SquareRoot {}

pub enum OperatorType {
    Operator,
    Function,
}

#[derive(Clone, Copy)]
pub struct Operator {
    pub(crate) priority: i32,
    pub(crate) char: &'static char,
    pub(crate) operator_type: OperatorType,
    pub calculable: Calculable,
}

impl Clone for OperatorType {
    fn clone(&self) -> Self {
        return *self;
    }
}

impl Copy for OperatorType {

}

pub type LargeDecimal = f64;

impl to_underlying_type for LargeDecimal {
    fn to_f64(self) -> f64 {
        return self as f64;
    }
}

trait to_underlying_type {
    fn to_f64(self) -> f64;
}


impl Addition {
    pub fn new() -> Operator {
        Operator {
            priority: 4,
            char: &'+',
            operator_type: OperatorType::Operator,
            calculable: |values| {
                return (values.get(0).unwrap() + values.get(1).unwrap()).to_string().chars().collect();
            },
        }
    }
}

impl Subtraction {
   pub fn new() -> Operator {
        Operator {
            priority: 4,
            char: &'-',
            operator_type: OperatorType::Operator,
            calculable: |values| {
                return (values.get(0).unwrap() - values.get(1).unwrap()).to_string().chars().collect();
            },
        }
    }
}

impl Multiplication {
    pub fn new() -> Operator {
        Operator {
            priority: 2,
            char: &'*',
            operator_type: OperatorType::Operator,
            calculable: |values| {
                return (values.get(0).unwrap() * values.get(1).unwrap()).to_string().chars().collect();
            },
        }
    }
}

impl Division {
    pub fn new() -> Operator {
        Operator {
            priority: 2,
            char: &'/',
            operator_type: OperatorType::Operator,
            calculable: |values| {
                let val1 = values.get(0).unwrap();
                let val2 = values.get(1).unwrap();

                if val1.eq(&0.0) || val2.eq(&0.0) {
                    return 0.to_string().chars().collect();
                }

                return (val1 / val2).to_string().chars().collect();
            },
        }
    }
}

impl PowerOf {
    pub fn new() -> Operator {
        Operator {
            priority: 1,
            char: &'^',
            operator_type: OperatorType::Operator,
            calculable: |values| {
                let val1 = values.get(0).unwrap();
                let val2 = values.get(1).unwrap();

                return (val1.powf(val2.to_f64())).to_string().chars().collect()
            },
        }
    }
}

impl SquareRoot {
    pub fn new() -> Operator {
        Operator {
            priority: 1,
            char: &'√',
            operator_type: OperatorType::Function,
            calculable: |values| {
                let val1 = values.get(0).unwrap();
                return (val1.sqrt()).to_string().chars().collect()
            },
        }
    }
}