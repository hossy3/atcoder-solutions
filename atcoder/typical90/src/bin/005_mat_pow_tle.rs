use proconio::input;

type Mint = ac_library::ModInt1000000007;

fn mat_mul(matrix0: &[Vec<Mint>], matrix1: &[Vec<Mint>]) -> Vec<Vec<Mint>> {
    let h0 = matrix0.len();
    let w0 = matrix0[0].len();
    let h1 = matrix1.len();
    let w1 = matrix1[0].len();
    assert!(h0 > 0);
    assert!(w0 > 0);
    assert!(h1 > 0);
    assert!(w1 > 0);
    assert_eq!(w0, h1);

    let mut matrix = vec![vec![Mint::new(0); w1]; h0];
    for i in 0..h0 {
        for j in 0..w1 {
            for k in 0..w0 {
                matrix[i][j] += matrix0[i][k] * matrix1[k][j];
            }
        }
    }
    matrix
}

fn mat_pow(matrix: &Vec<Vec<Mint>>, mut exp: usize) -> Vec<Vec<Mint>> {
    let h = matrix.len();
    let w = matrix[0].len();
    assert!(h > 0);
    assert_eq!(h, w);
    assert_ne!(exp, 0);

    let mut base = matrix.clone();

    while exp & 1 == 0 {
        base = mat_mul(&base, &base);
        exp >>= 1;
    }
    if exp == 1 {
        return base;
    }

    let mut acc = base.clone();
    while exp > 1 {
        exp >>= 1;
        base = mat_mul(&base, &base);
        if exp & 1 == 1 {
            acc = mat_mul(&acc, &base);
        }
    }
    acc
}

fn main() {
    input! {
        n: usize,
        b: usize,
        k: usize,
        c: [usize; k],
    }

    let mut matrix = vec![vec![Mint::new(0); b]; b];
    for i in 0..b {
        for &x in &c {
            let j = (i * 10 + x) % b; // 余りiを10倍してxを足すと、次の余りに
            matrix[j][i] += Mint::new(1);
        }
    }
    let matrix = mat_pow(&matrix, n);

    let mut vec = vec![vec![Mint::new(0)]; b];
    vec[0][0] = Mint::new(1);
    let result = (mat_mul(&matrix, &vec))[0][0];

    println!("{result}");
}
