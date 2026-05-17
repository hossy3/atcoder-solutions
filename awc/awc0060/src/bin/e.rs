use proconio::{input, marker::Usize1};

// 10^(2^k) % m を返す
fn build_pow10(nbits: usize, m: usize) -> Vec<usize> {
    let mut pow10 = vec![0; nbits];
    pow10[0] = 10 % m;
    for k in 0..(nbits - 1) {
        pow10[k + 1] = pow10[k].pow(2) % m;
    }
    pow10
}

// 返り値: [k][i] = (i から 2^k 回移動した先, その時の 2^k 桁の値の mod m)
fn build_doubling(k: usize, dp: &[(usize, usize)], m: usize) -> Vec<Vec<(usize, usize)>> {
    let n = dp.len();
    let nbits = k.ilog2() as usize + 1;
    let mut dbl = vec![vec![(0, 0); n]; nbits];
    for (i, &(d, p)) in dp.iter().enumerate() {
        dbl[0][i] = (p, d % m);
    }
    let pow10 = build_pow10(nbits, m);

    for k in 0..(nbits - 1) {
        for i in 0..n {
            let (i0, v0) = dbl[k][i]; // i から 2^k 回移動した先
            let (i1, v1) = dbl[k][i0]; // i0 から 2^k 回移動した先
            dbl[k + 1][i] = (i1, (v0 * pow10[k] + v1) % m); // i から 2^(k+1) 回移動した先
        }
    }
    dbl
}

fn main() {
    input! {
        n: usize,
        q: usize,
        m: usize,
        dp: [(usize, Usize1); n],
        sk: [(Usize1, usize); q],
    }

    let k = sk.iter().map(|&(_, k)| k.max(1)).max().unwrap();
    let dbl = build_doubling(k, &dp, m);
    let pow10 = build_pow10(dbl.len(), m);

    for &(mut s, k) in &sk {
        let mut result = 0;
        for (i, dbl) in dbl.iter().enumerate() {
            if k & (1 << i) != 0 {
                let (i0, v0) = dbl[s]; // s から 2^i 回移動した先
                result = (result * pow10[i] + v0) % m;
                s = i0;
            }
        }
        println!("{result}");
    }
}
