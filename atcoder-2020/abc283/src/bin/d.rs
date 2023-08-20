use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut stack = vec![HashSet::new()];
    let mut set = HashSet::new();
    for i in 0..(s.len()) {
        let c = s[i];
        match c {
            '(' => {
                stack.push(HashSet::new());
            }
            ')' => {
                let s = stack.pop().unwrap();
                for c in &s {
                    set.remove(c);
                }
            }
            _ => {
                if set.contains(&c) {
                    println!("No");
                    return;
                }
                set.insert(c);
                stack.last_mut().unwrap().insert(c);
            }
        }
    }

    println!("Yes");
}
