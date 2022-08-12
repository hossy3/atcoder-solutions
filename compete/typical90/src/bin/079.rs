use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [[i64; w]; h],
        b: [[i64; w]; h],
    }
    let mut count = 0;
    for i in 0..(h - 1) {
        for j in 0..(w - 1) {
            let diff = b[i][j] - a[i][j];
            count += diff.abs();
            for k in 0..2 {
                for l in 0..2 {
                    a[i + k][j + l] += diff;
                }
            }
        }
    }
    let mut yes = true;
    for i in 0..h {
        if a[i][w - 1] != b[i][w - 1] {
            yes = false;
            break;
        }
    }
    for i in 0..w {
        if a[h - 1][i] != b[h - 1][i] {
            yes = false;
            break;
        }
    }
    println!("{}", if yes { "Yes" } else { "No" });
    if yes {
        println!("{}", count);
    }
}
