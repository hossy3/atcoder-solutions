use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }
    let mut a = Vec::new();
    let mut m = HashMap::new();

    for i in 0..q {
        input! {
            key: String,
        }
        match key.as_str() {
            "ADD" => {
                input! {
                    x: i64,
                }
                a.push(x);
            }
            "DELETE" => {
                a.pop();
            }
            "SAVE" => {
                input! {
                    y: usize,
                }
                m.insert(y, a.clone());
            }
            "LOAD" => {
                input! {
                    z: usize,
                }
                a = m.get(&z).unwrap_or(&Vec::new()).clone();
            }
            _ => {}
        }
        if i + 1 == q {
            println!("{}", a.last().unwrap_or(&-1));
        } else {
            print!("{} ", a.last().unwrap_or(&-1));
        }
    }
}
