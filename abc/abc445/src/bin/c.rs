use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut state: Vec<_> = a.clone();
    for _ in 0..100 {
        let mut state0 = state.clone();
        for _ in 0..9 {
            for i in 0..n {
                state0[i] = state[state0[i]];
            }
        }
        state = state0;
    }
    let result = state.iter().map(|&x| x + 1).join(" ");
    println!("{result}");
}
