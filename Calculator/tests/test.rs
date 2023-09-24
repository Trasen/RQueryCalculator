use RQueryCalculator::calc;

#[test]
fn addition() {
    assert_eq!("2", calc(&String::from("1+1")));
}
#[test]
fn additionDecimal() {
    assert_eq!("1", calc(&String::from("0.5+0.5")));
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
#[test]
fn largeNumbers() {
    assert_eq!("999999999999900000000000000000", calc(&String::from("9999999999999*99999999999999999")));
}