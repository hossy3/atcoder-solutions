use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        b: [[usize; n]; n],
    }
    let x0 = (0..(n * n)).all(|i| {
        let (i, j) = (i / n, i % n);
        a[i][j] == 0 || b[i][j] == 1
    });
    let x1 = (0..(n * n)).all(|i| {
        let (i, j) = (i / n, i % n);
        a[i][j] == 0 || b[n - j - 1][i] == 1
    });
    let x2 = (0..(n * n)).all(|i| {
        let (i, j) = (i / n, i % n);
        a[i][j] == 0 || b[n - i - 1][n - j - 1] == 1
    });
    let x3 = (0..(n * n)).all(|i| {
        let (i, j) = (i / n, i % n);
        a[i][j] == 0 || b[j][n - i - 1] == 1
    });
    let yes = x0 || x1 || x2 || x3;
    println!("{}", if yes { "Yes" } else { "No" });
}
