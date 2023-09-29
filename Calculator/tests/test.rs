use RQueryCalculator::calc;

#[test]
fn addition() {
    assert_eq!("2", calc(String::from("1+1")));
}

#[test]
fn no_calculations_should_return_same_numbers() {
    assert_eq!("11", calc(String::from("11")));
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
fn negative_values_should_be_handled_properly() {
    assert_eq!("25", calc(String::from("-5*-5")));
}
#[test]
fn negative_values_should_be_handled_properly2() {
    assert_eq!("61.5", calc(String::from("-5*-5*-5/-2+-1")));
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
fn division2() {
    assert_eq!("2700000", calc(String::from("9000*(600/2)")));
}

#[test]
fn powerof() {
    assert_eq!("2342758.075390642", calc(String::from("55^3.66")));
}

#[test]
fn chained_calculations_should_be_supported() {
    assert_eq!("8", calc(String::from("4+5-1")));
}

#[test]
fn multiple_priority_calculations_should_be_supported() {
    assert_eq!("51199999999999940", calc(String::from("4000^4*200+(-55-5)")));
}
#[test]
fn multiple_priority_calculations_should_be_supported2() {
    assert_eq!("0", calc(String::from("(4000^4*200+(5-5))*0")));
}

#[test]
fn large_numbers() {
    assert_eq!("999999999999900000000000000000", calc(String::from("9999999999999*99999999999999999")));
}

#[test]
fn regular_text_should_be_allowed() {
    assert_eq!("HejHå 0 HejHå", calc(String::from("HejHå (4000^4*200+(5-5))*0 HejHå")));
}


#[test]
fn regular_text_should_be_allowed2() {
    assert_eq!("Once upon a time there were 16 dwarfs digging 256 holes", calc(String::from("Once upon a time there were 2*8 dwarfs digging 2^8 holes")));
}