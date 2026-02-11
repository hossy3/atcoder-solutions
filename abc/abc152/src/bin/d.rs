use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut m = [[0usize; 10]; 10]; // m[先頭の数字][末尾の数字] = 個数
    for i in 1..=n {
        let x = i / 10usize.pow(i.ilog10());
        let y = i % 10;
        m[x][y] += 1;
    }
    let mut result = 0usize;
    for i in 1..=9 {
        for j in 1..=9 {
            result += m[i][j] * m[j][i];
        }
    }
    println!("{result}");
}
