use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }
    let mut v = VecDeque::new();
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                x: usize,
                c: usize,
            }
            v.push_back((x, c));
        } else {
            input! {
                mut c: usize,
            }
            let mut sum = 0;
            while c > 0 {
                let (x, c0) = v.pop_front().unwrap();
                if c >= c0 {
                    sum += x * c0;
                    c -= c0;
                } else {
                    sum += x * c;
                    v.push_front((x, c0 - c));
                    c = 0;
                }
            }
            println!("{}", sum);
        }
    }
}
