use itertools::Itertools;
use proconio::{input, marker::Chars};

const MOD: usize = 998244353;
const M: usize = 6;
const M0: usize = 1 << M;

fn chars_to_nums(s: &[char]) -> (usize, usize) {
    let mut i = 0;
    for &c in s {
        i = i * 2 + if c == 'a' { 0 } else { 1 };
    }
    let j = 1 << s.len();
    (i, j)
}

fn solve_dp(s: &[(usize, usize)], n: usize) -> [usize; M0] {
    let mut dp = vec![[0; M0]; n + 1];
    dp[0][0] = 1;
    for i in 0..n {
        for j in 0..M0 {
            if dp[i][j] == 0 {
                continue;
            }
            for k in 0..2 {
                let i0 = i + 1;
                let j0 = ((j * 2) % M0) + k;
                let b = s.iter().any(|&(l, m)| (j0 % m == l) && (1 << i0) >= m);
                if !b {
                    dp[i0][j0] = (dp[i0][j0] + dp[i][j]) % MOD;
                }
            }
        }
    }

    dp[n].clone()
}

fn identity_mat() -> [[usize; M0]; M0] {
    let mut mat = [[0usize; M0]; M0];
    for i in 0..M0 {
        mat[i][i] = 1;
    }
    mat
}

fn build_mat(s: &[(usize, usize)]) -> [[usize; M0]; M0] {
    let mut mat = [[0usize; M0]; M0];
    for i in 0..M0 {
        mat[i][(i * 2) % M0] = 1;
        mat[i][(i * 2 + 1) % M0] = 1;
    }
    for i in 0..M0 {
        let b = s.iter().any(|&(l, m)| i % m == l);
        if b {
            for j in 0..M0 {
                mat[j][i] = 0;
            }
        }
    }
    mat
}

fn modmul_mat(mat0: &[[usize; M0]; M0], mat1: &[[usize; M0]; M0]) -> [[usize; M0]; M0] {
    let mut mat = [[0usize; M0]; M0];
    for i in 0..M0 {
        for j in 0..M0 {
            for k in 0..M0 {
                mat[i][j] = (mat[i][j] + mat0[i][k] * mat1[k][j]) % MOD;
            }
        }
    }
    mat
}

fn modmul_v_mat(v0: &[usize; M0], mat1: &[[usize; M0]; M0]) -> [usize; M0] {
    let mut v = [0usize; M0];
    for i in 0..M0 {
        for j in 0..M0 {
            v[i] = (v[i] + v0[j] * mat1[j][i]) % MOD;
        }
    }
    v
}

fn solve(n: usize, s: &[(usize, usize)]) -> usize {
    let v = solve_dp(s, n.min(M));
    if n <= M {
        let result = v.iter().sum();
        return result;
    }

    let mut mat = identity_mat();
    let mut mat0 = build_mat(s);
    let mut m = n - M;
    while m > 0 {
        if m % 2 == 1 {
            mat = modmul_mat(&mat, &mat0);
        }
        mat0 = modmul_mat(&mat0, &mat0);
        m /= 2;
    }

    let v = modmul_v_mat(&v, &mat);
    let result = v.iter().sum::<usize>() % MOD;
    result
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; m],
    }

    let s = s.iter().map(|s| chars_to_nums(s)).collect_vec();
    let result = solve(n, &s);
    println!("{}", result);
}
