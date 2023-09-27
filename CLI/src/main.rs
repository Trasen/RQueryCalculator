fn main() {
    let calc = std::env::args().nth(1);
    let result = RQueryCalculator::calc(String::from(calc.unwrap()));
    println!("{}", result);
}
