use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut value = 0usize;
    let mut v = vec![];
    let mut map = HashMap::new();
    let mut i = 0; // 確認操作回数
    for _ in 0..n {
        input! {
            event: String,
        }
        match event.as_str() {
            "PUT" => {
                input! {
                    c: usize,
                }
                v.push(c);
                value ^= c;
            }
            "REMOVE" => {
                let c = v.pop().unwrap();
                value ^= c;
            }
            "LOOK" => {
                if let Some(x) = map.get(&value) {
                    println!("{}", x);
                } else {
                    println!("-1");
                }
                i += 1;
                map.insert(value, i);
            }
            _ => unreachable!(),
        }
    }
}
