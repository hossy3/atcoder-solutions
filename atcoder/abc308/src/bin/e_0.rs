use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        s: Chars,
    }

    let mut x = vec![[0, 0, 0]; n + 1]; // 0, 1, 2
    for i in (0..n).rev() {
        x[i] = x[i + 1];
        if s[i] != 'X' {
            continue;
        }
        x[i][a[i]] += 1;
    }

    let mut e = vec![[0, 0, 0, 0, 0, 0]; n + 1]; // 0-0, 0-1, 0-2, 1-1, 1-2, 2-2
    for i in (0..n).rev() {
        e[i] = e[i + 1];
        if s[i] != 'E' {
            continue;
        }
        match a[i] {
            0 => {
                e[i][0] += x[i + 1][0];
                e[i][1] += x[i + 1][1];
                e[i][2] += x[i + 1][2];
            }
            1 => {
                e[i][1] += x[i + 1][0];
                e[i][3] += x[i + 1][1];
                e[i][4] += x[i + 1][2];
            }
            _ => {
                e[i][2] += x[i + 1][0];
                e[i][4] += x[i + 1][1];
                e[i][5] += x[i + 1][2];
            }
        }
    }

    let mut result = 0usize;
    for i in 0..n {
        if s[i] != 'M' {
            continue;
        }
        result += match a[i] {
            0 => e[i][0] * 1 + e[i][1] * 2 + e[i][2] * 1 + e[i][3] * 2 + e[i][4] * 3 + e[i][5] * 1,
            1 => e[i][0] * 2 + e[i][1] * 2 + e[i][2] * 3 + e[i][3] * 0 + e[i][4] * 0 + e[i][5] * 0,
            _ => e[i][0] * 1 + e[i][1] * 3 + e[i][2] * 1 + e[i][3] * 0 + e[i][4] * 0 + e[i][5] * 0,
        }
    }

    println!("{}", result);
}
