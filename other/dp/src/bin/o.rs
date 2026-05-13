use proconio::input;

/// bit DP
fn bit_dp(a: &[Vec<u8>]) -> usize {
    type Mint = ac_library::ModInt1000000007;

    let bit_test = |bits: usize, i: usize| bits & (1 << i) != 0;

    let n = a.len();
    let mut state = vec![Mint::new(0); 1 << n]; // ペアになった女性の集合
    state[0] += 1;

    for bits in 0usize..((1 << n) - 1) {
        let i = bits.count_ones() as usize;
        for j in 0..n {
            if a[i][j] == 1 && !bit_test(bits, j) {
                let i0 = bits | (1 << j);
                let x = state[bits];
                state[i0] += x;
            }
        }
    }
    state[(1 << n) - 1].val() as usize
}

fn main() {
    input! {
        n: usize,
        a: [[u8; n]; n],
    }
    let result = bit_dp(&a);
    println!("{result}");
}
