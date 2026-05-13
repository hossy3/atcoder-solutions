use ac_library::FenwickTree;
use proconio::input;

type Mint = ac_library::ModInt1000000007;

/// DP + Fenwick Tree
fn solve_dp(k: usize, a: &[usize]) -> Mint {
    let mut state = FenwickTree::new(k + 1, Mint::new(0));
    state.add(k, 1);
    for &a in a {
        let mut next = FenwickTree::new(k + 1, Mint::new(0));
        for i in 0..=k {
            let i1 = (i + a).min(k);
            let x = state.sum(i..=i1);
            next.add(i, x);
        }
        state = next;
    }
    state.accum(1)
    // state.sum(0..1)
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let result = solve_dp(k, &a);
    println!("{result}");
}
