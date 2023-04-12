use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut a = vec![(-1, 0)];
    let mut m = HashMap::new();
    let mut cur = 0;

    for i in 0..q {
        input! {
            key: String,
        }
        match key.as_str() {
            "ADD" => {
                input! {
                    x: i64,
                }
                a.push((x, cur));
                cur = a.len() - 1;
            }
            "DELETE" => {
                cur = a[cur].1;
            }
            "SAVE" => {
                input! {
                    y: usize,
                }
                m.insert(y, cur);
            }
            "LOAD" => {
                input! {
                    z: usize,
                }
                cur = *m.get(&z).unwrap_or(&0);
            }
            _ => {}
        }
        if i + 1 == q {
            println!("{}", a[cur].0);
        } else {
            print!("{} ", a[cur].0);
        }
    }
}
