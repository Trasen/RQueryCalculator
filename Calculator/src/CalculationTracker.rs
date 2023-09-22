use std::collections::HashMap;
use crate::{Addition, Operator};

pub fn FindNextOperation(query: &String, operatorCommands: HashMap<char, Operator>) -> Option<OperationTracker> {
    let mut operator: Option<Operator> = None;
    let mut leftStartOfOperation: Option<usize> = None;
    let mut leftEndOfOperation: Option<usize> = None;
    let mut rightStartOfOperation: Option<usize> = None;
    let mut rightEndOfOperation: Option<usize> = None;


    let queryChars = query.as_bytes();
    let queryEnumerator = queryChars.iter().enumerate();

    for (index, u8char) in queryEnumerator {
        let char = char::from(*u8char);

        if operatorCommands.contains_key(&char) {
            leftEndOfOperation = Some(index - 1);
            rightStartOfOperation = Some(index + 1);
            operator = Some(operatorCommands[&char]);

            // backtrack over the query to find all numbers that need to be processed
            let mut i = index - 1;
            while i >= 0 {
                let innerChar: char = char::from(queryChars[i]);

                if !innerChar.is_numeric() {
                    leftStartOfOperation = Some(i);
                    i = i - 1;
                    break;
                }


                if i == 0 {
                    leftStartOfOperation = Some(i);
                    break;
                }
                i = i - 1;
            }

            // move forward in the query to find all numbers that need to be processed
            let mut u = index + 1;
            while u < query.len() {
                let innerChar: char = char::from(queryChars[u]);

                if !innerChar.is_numeric() {
                    rightEndOfOperation = Some(u - 1);
                    u = u + 1;
                    break;
                }


                if u == query.len() - 1 {
                    rightEndOfOperation = Some(u);
                    break;
                }
                u = u + 1;
            }
        }
    }

    for x in Vec::from([leftStartOfOperation.is_none(), leftEndOfOperation.is_none(), rightStartOfOperation.is_none(), rightEndOfOperation.is_none(), operator.is_none()]) {
            if x == true {
                return None;
            }
        }

    let result = Some(OperationTracker {
        leftStart: leftStartOfOperation.unwrap(),
        leftEnd: leftEndOfOperation.unwrap(),
        rightStart: rightStartOfOperation.unwrap(),
        rightEnd: rightEndOfOperation.unwrap(),
        calculable: operator.unwrap(),
    });

    return result;
}

pub struct OperationTracker {
    pub leftStart: usize,
    pub leftEnd: usize,
    pub rightStart:usize,
    pub rightEnd: usize,
    pub calculable: Operator,
}

#[test]
fn operationIsParsedProperly() {
    let result = FindNextOperation(&String::from("1+1"), HashMap::from([('+', Addition::new())]));
    assert_eq!(result.is_none(), false);


    let operationTracker = result.unwrap();

    assert_eq!(0, operationTracker.leftStart);
    assert_eq!(0, operationTracker.leftEnd);
    assert_eq!(2, operationTracker.rightStart);
    assert_eq!(2, operationTracker.rightEnd);
}

#[test]
fn queryHasNoCalculableValues() {
    let result = FindNextOperation(&String::from("123123"), HashMap::from([('+', Addition::new())]));
    assert_eq!(result.is_none(), true);
}