use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        t: usize,
        cv: [(usize, usize); n],
    }

    let s = s - t;
    let mut state = vec![0usize; s + 1]; // [i] お金を i 使った時の満足度の最大
    for &(c, v) in &cv {
        if c > s {
            continue;
        }
        for i in (0..=(s - c)).rev() {
            state[i + c] = state[i + c].max(state[i] + v);
        }
    }
    let result = state[s];
    println!("{result}");
}
