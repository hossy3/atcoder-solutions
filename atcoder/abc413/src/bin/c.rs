use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut queue = VecDeque::new();
    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            1 => {
                input! {
                    c: usize,
                    x: usize,
                }
                queue.push_back((c, x));
            }
            2 => {
                input! {
                    mut k: usize,
                }
                let mut result = 0usize;
                while let Some((c, x)) = queue.pop_front() {
                    if k >= c {
                        result += c * x;
                        k -= c;
                        if k == 0 {
                            break;
                        }
                    } else {
                        result += k * x;
                        queue.push_front((c - k, x));
                        break;
                    }
                }
                println!("{result}");
            }
            _ => unreachable!(),
        }
    }
}
