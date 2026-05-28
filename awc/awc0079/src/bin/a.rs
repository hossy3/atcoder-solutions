use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    let yes = (0..n).all(|i| {
        (0..m).all(|j| {
            if s[i][j] != '#' {
                return true;
            }
            let mut count = 0;
            if i > 0 && s[i - 1][j] == '#' {
                count += 1;
            }
            if i < n - 1 && s[i + 1][j] == '#' {
                count += 1;
            }
            if j > 0 && s[i][j - 1] == '#' {
                count += 1;
            }
            if j < m - 1 && s[i][j + 1] == '#' {
                count += 1;
            }
            1 <= count && count <= 3
        })
    });
    println!("{}", if yes { "Yes" } else { "No" });
}
