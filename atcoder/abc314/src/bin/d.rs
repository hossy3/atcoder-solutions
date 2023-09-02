use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
        txc: [(usize, usize, char); q],
    }

    let last23 = txc.iter().rposition(|&(t, _, _)| t != 1);
    for (i, &(t, x, c)) in txc.iter().enumerate() {
        match t {
            1 => {
                s[x - 1] = c;
            }
            2 => {
                if Some(i) == last23 {
                    for i in 0..n {
                        s[i] = s[i].to_ascii_lowercase();
                    }
                }
            }
            3 => {
                if Some(i) == last23 {
                    for i in 0..n {
                        s[i] = s[i].to_ascii_uppercase();
                    }
                }
            }
            _ => panic!(),
        }
    }

    println!("{}", s.iter().join(""));
}
