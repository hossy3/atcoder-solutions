use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        _: usize,
        mut s: [Chars; h],
        t: Chars,
    }

    let mut x = 0;
    let mut y = 0;
    for c in t {
        s[x][y] = '.';
        match c {
            'U' => {
                x = x.saturating_sub(1);
            }
            'D' => {
                x = (x + 1).min(h - 1);
            }
            'L' => {
                y = y.saturating_sub(1);
            }
            'R' => {
                y = (y + 1).min(w - 1);
            }
            _ => {
                unreachable!()
            }
        }
    }
    s[x][y] = '.';

    let result = s
        .iter()
        .map(|s| s.iter().filter(|&&c| c == '#').count())
        .sum::<usize>();
    println!("{result}");
}
