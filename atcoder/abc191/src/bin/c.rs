use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let mut result = 0;
    for i in 1..(h - 1) {
        for j in 1..(w - 1) {
            if s[i][j] == '#' && s[i - 1][j - 1] == '.' && s[i][j - 1] == s[i - 1][j] {
                result += 1;
            }
            if s[i][j] == '#' && s[i + 1][j - 1] == '.' && s[i][j - 1] == s[i + 1][j] {
                result += 1;
            }
            if s[i][j] == '#' && s[i - 1][j + 1] == '.' && s[i][j + 1] == s[i - 1][j] {
                result += 1;
            }
            if s[i][j] == '#' && s[i + 1][j + 1] == '.' && s[i][j + 1] == s[i + 1][j] {
                result += 1;
            }
        }
    }
    println!("{result}");
}
