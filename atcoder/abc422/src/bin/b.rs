use proconio::{input, marker::Chars};

fn f(s: &[Vec<char>]) -> bool {
    let h = s.len();
    let w = s[0].len();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] != '#' {
                continue;
            }

            let mut count = 0;
            if i > 0 && s[i - 1][j] == '#' {
                count += 1;
            }
            if i + 1 < h && s[i + 1][j] == '#' {
                count += 1;
            }
            if j > 0 && s[i][j - 1] == '#' {
                count += 1;
            }
            if j + 1 < w && s[i][j + 1] == '#' {
                count += 1;
            }
            if count != 2 && count != 4 {
                return false;
            }
        }
    }
    true
}

fn main() {
    input! {
        h: usize,
        _w: usize,
        s: [Chars; h],
    }

    let yes = f(&s);
    println!("{}", if yes { "Yes" } else { "No" });
}
