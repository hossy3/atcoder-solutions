use itertools::Itertools;
use ndarray::{Array1, Array2};
use proconio::{input, marker::Chars};

const MOD: usize = 998244353;
const M: usize = 6;
const N: usize = 1 << M;

fn chars_to_nums(s: &[char]) -> (usize, usize) {
    let mut i = 0;
    for &c in s {
        i = i * 2 + if c == 'a' { 0 } else { 1 };
    }
    let j = 1 << s.len();
    (i, j)
}

#[test]
fn test_chars_to_nums() {
    assert_eq!(chars_to_nums(&"aaaa".chars().collect_vec()), (0, 16));
    assert_eq!(chars_to_nums(&"aaab".chars().collect_vec()), (1, 16));
    assert_eq!(chars_to_nums(&"aabb".chars().collect_vec()), (3, 16));
    assert_eq!(chars_to_nums(&"abbb".chars().collect_vec()), (7, 16));
    assert_eq!(chars_to_nums(&"bbbb".chars().collect_vec()), (15, 16));
}

fn solve_dp(s: &[(usize, usize)], n: usize) -> Vec<usize> {
    let mut dp = vec![vec![0; N]; n + 1];
    dp[0][0] = 1;
    for i in 0..n {
        for j in 0..N {
            if dp[i][j] == 0 {
                continue;
            }
            for k in 0..2 {
                let i0 = i + 1;
                let j0 = ((j * 2) % N) + k;
                let b = s.iter().any(|&(l, m)| (j0 % m == l) && (1 << i0) >= m);
                if !b {
                    dp[i0][j0] = (dp[i0][j0] + dp[i][j]) % MOD;
                }
            }
        }
    }

    dp[n].clone()
}

fn build_mat(s: &[(usize, usize)]) -> Array2<u128> {
    let mut mat = Array2::zeros((N, N));
    for i in 0..N {
        mat[(i, (i * 2) % N)] = 1;
        mat[(i, (i * 2 + 1) % N)] = 1;
    }
    for i in 0..N {
        let b = s.iter().any(|&(l, m)| i % m == l);
        if b {
            for j in 0..N {
                mat[(j, i)] = 0;
            }
        }
    }
    mat
}

fn mod_mat(mat: &mut Array2<u128>) {
    for i in 0..N {
        for j in 0..N {
            mat[(i, j)] %= MOD as u128;
        }
    }
}

fn solve(n: usize, s: &[(usize, usize)]) -> usize {
    let v = solve_dp(s, n.min(M));
    if n <= M {
        let result = v.iter().sum();
        return result;
    }

    let mut mat = Array2::eye(N);
    let mut mat0 = build_mat(s);
    let mut m = n - M;
    while m > 0 {
        if m % 2 == 1 {
            mat = mat.dot(&mat0);
            mod_mat(&mut mat);
        }
        mat0 = mat0.dot(&mat0);
        mod_mat(&mut mat0);
        m /= 2;
    }

    let mut matu = Array2::eye(N);
    for i in 0..N {
        for j in 0..N {
            matu[(i, j)] = mat[(i, j)] as usize;
        }
    }

    let v: Array1<u128> = v.iter().map(|&x| x as u128).collect_vec().into();
    let result = v.dot(&mat).iter().sum::<u128>() % MOD as u128;
    result as usize
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
