use RQueryCalculator::calc;

#[test]
fn addition() {
    assert_eq!("2", calc(&String::from("1+1")));
}

#[test]
fn multiplication() {
    assert_eq!("25", calc(&String::from("5*5")));
}

#[test]
fn subtraction() {
    assert_eq!("0", calc(&String::from("1-1")));
}

#[test]
fn division() {
    assert_eq!("1", calc(&String::from("2/2")));
}

#[test]
fn chainedCalculationsShouldBeSupported() {
    assert_eq!("8", calc(&String::from("4+5-1")));
}

#[test]
fn multiplePriorityCalculationsShouldBeSupported() {
    assert_eq!("800000", calc(&String::from("4000*200+5-5")));
}