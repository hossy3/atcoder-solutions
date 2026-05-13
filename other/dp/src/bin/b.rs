use proconio::input;

// 配るDP (Push DP)

fn main() {
    input! {
        n: usize,
        k: usize,
        h: [usize; n],
    }

    let mut state = vec![usize::MAX; n];
    state[0] = 0;
    for i in 0..(n - 1) {
        for j in (i + 1)..(i + k + 1).min(n) {
            state[j] = state[j].min(state[i] + h[i].abs_diff(h[j]));
        }
    }
    println!("{}", state[n - 1]);
}
