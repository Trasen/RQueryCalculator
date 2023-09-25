use RQueryCalculator::calc;

#[test]
fn addition() {
    assert_eq!("2", calc(String::from("1+1")));
}

#[test]
fn grouped_calculations_should_be_supported() {
    assert_eq!("5", calc(String::from("(1+1)+(2+1)")));
}

#[test]
fn nested_grouped_calculations_should_be_supported() {
    assert_eq!("14", calc(String::from("(1+1*(5+5))+(2+1)")));
}
#[test]
fn addition_decimal() {
    assert_eq!("1", calc(String::from("0.5+0.5")));
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

#[test]
fn chained_calculations_should_be_supported() {
    assert_eq!("8", calc(String::from("4+5-1")));
}

#[test]
fn multiple_priority_calculations_should_be_supported() {
    assert_eq!("800000", calc(String::from("4000*200+(5-5)")));
}

#[test]
fn large_numbers() {
    assert_eq!("999999999999900000000000000000", calc(String::from("9999999999999*99999999999999999")));
}