use proconio::{input, marker::Chars};

fn ctoi(c: char) -> isize {
    c as isize - '0' as isize
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        s: [Chars; n],
    }

    let mut cum = vec![vec![0isize; m + 1]; n + 1];
    for i in 0..n {
        for j in 0..m {
            cum[i + 1][j + 1] = ctoi(s[i][j]);
        }
    }
    for i in 0..n {
        for j in 0..(m + 1) {
            cum[i + 1][j] += cum[i][j];
        }
    }
    for i in 0..(n + 1) {
        for j in 0..m {
            cum[i][j + 1] += cum[i][j];
        }
    }

    let mut result = -1isize;
    for h in 1..=n {
        let w = k / h;
        if h * w < k || w > m {
            continue;
        }
        for i in 0..=n - h {
            for j in 0..=m - w {
                let x = cum[i + h][j + w] - cum[i][j + w] - cum[i + h][j] + cum[i][j];
                result = result.max(x);
            }
        }
    }
    println!("{result}");
}
