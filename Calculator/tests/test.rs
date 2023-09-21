use Calculator::calc;

#[test]
fn addition() {
    assert_eq!("2", calc(String::from("1+1")));
}

#[test]
fn multiplication() {
    assert_eq!("25", calc(String::from("5*5")));
}

#[test]
fn subtraction() {
    assert_eq!("0", calc(String::from("1-1")));
}

#[test]
fn division() {
    assert_eq!("1", calc(String::from("2/2")));
}
