use std::io;
use std::io::{BufRead, Write};
use RQueryCalculator::calc;

fn main() {

    let calculatablevalue = std::env::args().nth(1);
    let mut finished = String::from("");
    match calculatablevalue {
        None => {
            io::stdin().lock().lines().fold((), |acc, line| ({

                match line {
                    Ok(li) => {
                        let mut res = calc(li);
                        res.insert_str(res.len(), "\n");
                        finished.insert_str(finished.len(), &res);
                    }
                    Err(_) => {
                        dbg!("Error");
                    }
                }
            }));
        }
        Some(some) => {
            let result = RQueryCalculator::calc(String::from(some));
            finished = result;
        }
    };

    io::stdout().lock().write_all(finished.as_bytes());
}
