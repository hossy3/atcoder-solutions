use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars],
    }
    let mut set = HashSet::new();
    for s in s {
        match s[0] {
            'H' | 'D' | 'C' | 'S' => {}
            _ => {
                println!("No");
                return;
            }
        }
        match s[1] {
            'A' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'T' | 'J' | 'Q' | 'K' => {}
            _ => {
                println!("No");
                return;
            }
        }
        if set.contains(&s) {
            println!("No");
            return;
        }
        set.insert(s);
    }
    println!("Yes");
}
