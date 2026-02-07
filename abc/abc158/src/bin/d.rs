use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        q: usize,
    }

    let mut b = true; // 順方向 true, 逆方向 false とする
    let mut s = VecDeque::from(s.clone());
    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            1 => b = !b,
            2 => {
                input! {
                    f: u8,
                    c: char,
                }
                if (b && f == 1) || (!b && f == 2) {
                    s.push_front(c);
                } else {
                    s.push_back(c);
                }
            }
            _ => unreachable!(),
        }
    }

    let result = if b {
        s.iter().collect::<String>()
    } else {
        s.iter().rev().collect::<String>()
    };
    println!("{result}");
}
