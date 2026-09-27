use std::collections::HashSet;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut events = vec![(2, usize::MAX, 'a')];
    let mut tiles = vec![false; n];
    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            1 => {
                input! {
                    x: Usize1,
                }
                events.push((t, x, ' '));
                tiles[x] = !tiles[x];
            }
            2 => {
                input! {
                    c: char,
                }
                events.push((t, usize::MAX, c));
            }
            _ => unreachable!(),
        }
    }

    let mut set: HashSet<_> = (0..n).filter(|&i| !tiles[i]).collect();

    let mut colors = vec![' '; n];
    for &(t, x, c) in events.iter().rev() {
        match t {
            1 => {
                if colors[x] == ' ' {
                    if tiles[x] {
                        set.insert(x);
                    } else {
                        set.remove(&x);
                    }
                }
                tiles[x] = !tiles[x];
            }
            2 => {
                for &i in &set {
                    colors[i] = c;
                }
                set.clear();
            }
            _ => unreachable!(),
        }
    }

    println!("{}", colors.iter().join(""));
}
