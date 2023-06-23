use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let mut a = vec![vec![0; w + 2]; h + 2];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                a[i + 1][j + 1] = 1;
            }
        }
    }
    for i in 1..=h {
        for j in 1..=w {
            if a[i][j] == 0 {
                let c = a[i - 1][j] + a[i + 1][j] + a[i][j - 1] + a[i][j + 1];
                if c > 1 {
                    println!("{} {}", i, j);
                    return;
                }
            }
        }
    }

    panic!();
}
