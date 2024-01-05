use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        s: Chars,
    }

    let mut m = vec![[0, 0, 0]; n + 1]; // 0, 1, 2
    for i in 0..n {
        m[i + 1] = m[i];
        if s[i] != 'M' {
            continue;
        }
        m[i + 1][a[i]] += 1;
    }

    let mut e = vec![[0, 0, 0, 0, 0, 0]; n + 1]; // 0-0, 0-1, 0-2, 1-1, 1-2, 2-2
    for i in 0..n {
        e[i + 1] = e[i];
        if s[i] != 'E' {
            continue;
        }
        match a[i] {
            0 => {
                e[i + 1][0] += m[i][0];
                e[i + 1][1] += m[i][1];
                e[i + 1][2] += m[i][2];
            }
            1 => {
                e[i + 1][1] += m[i][0];
                e[i + 1][3] += m[i][1];
                e[i + 1][4] += m[i][2];
            }
            _ => {
                e[i + 1][2] += m[i][0];
                e[i + 1][4] += m[i][1];
                e[i + 1][5] += m[i][2];
            }
        }
    }

    let mut result = 0usize;
    for i in 0..n {
        if s[i] != 'X' {
            continue;
        }
        result += match a[i] {
            0 => {
                e[i + 1][0] * 1
                    + e[i + 1][1] * 2
                    + e[i + 1][2] * 1
                    + e[i + 1][3] * 2
                    + e[i + 1][4] * 3
                    + e[i + 1][5] * 1
            }
            1 => {
                e[i + 1][0] * 2
                    + e[i + 1][1] * 2
                    + e[i + 1][2] * 3
                    + e[i + 1][3] * 0
                    + e[i + 1][4] * 0
                    + e[i + 1][5] * 0
            }
            _ => {
                e[i + 1][0] * 1
                    + e[i + 1][1] * 3
                    + e[i + 1][2] * 1
                    + e[i + 1][3] * 0
                    + e[i + 1][4] * 0
                    + e[i + 1][5] * 0
            }
        }
    }

    println!("{}", result);
}
