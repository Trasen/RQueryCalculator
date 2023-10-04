use std::io;
use std::io::{BufRead, Write};

fn main() {

    let calc = std::env::args().nth(1);
    let mut finished = String::from("");
    match calc {
        None => {
            io::stdin().lock().lines().fold("".to_string(), |acc, line| (*{

                match line {
                    Ok(li) => {
                        finished.insert_str(finished.len(), &RQueryCalculator::calc(li));
                    }
                    Err(_) => {

                    }
                }
            }).parse().unwrap())
        }
        Some(some) => {
            let result = RQueryCalculator::calc(String::from(some));
            finished = result;
        }
    };

    io::stdout().lock().write_all(finished.as_bytes());
}
