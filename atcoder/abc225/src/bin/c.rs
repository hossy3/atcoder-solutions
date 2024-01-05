use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        b: [[usize; m]; n],
    }
    let yes = ((b[0][0] - 1) / 7 == (b[0][m - 1] - 1) / 7)
        && (0..(n - 1)).all(|i| b[i][0] + 7 == b[i + 1][0])
        && (0..n).all(|i| (0..(m - 1)).all(|j| b[i][j] + 1 == b[i][j + 1]));
    println!("{}", if yes { "Yes" } else { "No" });
}
