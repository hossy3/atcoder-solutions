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

fn identity_mat() -> [[usize; M0 * 2]; M0 * 2] {
    let mut mat = [[0usize; M0 * 2]; M0 * 2];
    for i in 0..(M0 * 2) {
        mat[i][i] = 1;
    }
    mat
}

fn build_mat(s: &[(usize, usize)]) -> [[usize; M0 * 2]; M0 * 2] {
    let mut mat = [[0usize; M0 * 2]; M0 * 2];

    // len: 1..=5 -> 2..=6
    for i in 0..M0 {
        mat[i][i * 2] = 1;
        mat[i][i * 2 + 1] = 1;
    }

    // len: 6 -> 6
    for i in 0..M0 {
        mat[i + M0][(i * 2) % M0 + M0] = 1;
        mat[i + M0][(i * 2 + 1) % M0 + M0] = 1;
    }

    // remove "s"
    for i in 1..=M {
        let i0 = 1 << i;
        for j in 0..i0 {
            let b = s.iter().any(|&(l, m)| m <= i0 && j % m == l);
            if b {
                for k in 0..(M0 * 2) {
                    mat[k][i0 + j] = 0;
                }
            }
        }
    }

    mat
}

fn modmul_mat(
    mat0: &[[usize; M0 * 2]; M0 * 2],
    mat1: &[[usize; M0 * 2]; M0 * 2],
) -> [[usize; M0 * 2]; M0 * 2] {
    let mut mat = [[0usize; M0 * 2]; M0 * 2];
    for i in 0..(M0 * 2) {
        for j in 0..(M0 * 2) {
            for k in 0..(M0 * 2) {
                mat[i][j] = (mat[i][j] + mat0[i][k] * mat1[k][j]) % MOD;
            }
        }
    }
    mat
}

fn modmul_v_mat(v0: &[usize; M0 * 2], mat1: &[[usize; M0 * 2]; M0 * 2]) -> [usize; M0 * 2] {
    let mut v = [0usize; M0 * 2];
    for i in 0..(M0 * 2) {
        for j in 0..(M0 * 2) {
            v[i] = (v[i] + v0[j] * mat1[j][i]) % MOD;
        }
    }
    v
}

fn solve(n: usize, s: &[(usize, usize)]) -> usize {
    let mut mat = identity_mat();
    let mut mat0 = build_mat(s);
    let mut m = n;
    while m > 0 {
        if m % 2 == 1 {
            mat = modmul_mat(&mat, &mat0);
        }
        mat0 = modmul_mat(&mat0, &mat0);
        m /= 2;
    }

    let mut v = [0usize; M0 * 2];
    v[1] = 1;
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
