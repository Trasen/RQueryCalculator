pub type Calculable = fn(values: Vec<LargeDecimal>) -> String;

pub struct Addition {}

pub struct Subtraction {}

pub struct Multiplication {}

pub struct Division {}

#[derive(Clone, Copy)]
pub struct Operator {
    pub(crate) priority: i32,
    pub(crate) char: &'static str,
    pub calculable: Calculable,
}


pub type LargeDecimal = f64;



impl Addition {
    pub fn new() -> Operator {
        Operator {
            priority: 4,
            char: "+",
            calculable: |values| {
                return (values.get(0).unwrap() + values.get(1).unwrap()).to_string();
            },
        }
    }
}

impl Subtraction {
   pub fn new() -> Operator {
        Operator {
            priority: 4,
            char: "-",
            calculable: |values| {
                return (values.get(0).unwrap() - values.get(1).unwrap()).to_string();
            },
        }
    }
}

impl Multiplication {
    pub fn new() -> Operator {
        Operator {
            priority: 2,
            char: "*",
            calculable: |values| {
                return (values.get(0).unwrap() * values.get(1).unwrap()).to_string();
            },
        }
    }
}

impl Division {
    pub fn new() -> Operator {
        Operator {
            priority: 2,
            char: "/",
            calculable: |values| {
                let val1 = values.get(0).unwrap();
                let val2 = values.get(1).unwrap();

                if val1.eq(&0.0) || val2.eq(&0.0) {
                    return 0.to_string();
                }

                return (values.get(0).unwrap() / values.get(1).unwrap()).to_string();
            },
        }
    }
}