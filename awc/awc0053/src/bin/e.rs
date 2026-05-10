use proconio::{input, marker::Usize1};

type Mint = ac_library::ModInt998244353;

/// 行列累乗
fn mat_pow(mat: &[Vec<Mint>], k: usize) -> Vec<Vec<Mint>> {
    fn mat_mul(mat0: &[Vec<Mint>], mat1: &[Vec<Mint>]) -> Vec<Vec<Mint>> {
        assert_eq!(mat0[0].len(), mat1.len());
        let mut mat = vec![vec![Mint::new(0); mat1[0].len()]; mat0.len()];
        for i in 0..mat0.len() {
            for j in 0..mat1[0].len() {
                for k in 0..mat0[0].len() {
                    mat[i][j] += mat0[i][k] * mat1[k][j];
                }
            }
        }
        mat
    }

    let mut mul: Vec<_> = mat.iter().cloned().collect();
    let mut result = vec![vec![Mint::new(0); mat[0].len()]; mat.len()];
    for i in 0..mul.len() {
        result[i][i] = Mint::new(1);
    }
    let mut k = k;
    while k != 0 {
        if k & 1 != 0 {
            result = mat_mul(&result, &mul);
        }
        mul = mat_mul(&mul, &mul);
        k >>= 1;
    }
    result
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        uvw: [(Usize1, Usize1, usize); m],
    }

    let mut mat = vec![vec![Mint::new(0); n]; n];
    for &(u, v, w) in &uvw {
        mat[u][v] = Mint::new(w);
    }
    mat = mat_pow(&mat, k);

    let result = mat[0][n - 1];
    println!("{result}");
}
