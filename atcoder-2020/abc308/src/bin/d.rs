use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn f(s: &[Vec<char>]) -> bool {
    if s[0][0] != 's' {
        return false;
    }

    let h = s.len();
    let w = s[0].len();
    let mut m = vec![vec![true; w + 2]; h + 2]; // visited
    for r in 0..h {
        for c in 0..w {
            m[r + 1][c + 1] = false;
        }
    }
    m[1][1] = true;
    let mut queue = VecDeque::new();
    queue.push_back((1, 1)); // row, column

    while let Some((r, c)) = queue.pop_back() {
        for i in 0..4 {
            let (r0, c0) = match i {
                0 => (r - 1, c),
                1 => (r + 1, c),
                2 => (r, c - 1),
                _ => (r, c + 1),
            };
            if m[r0][c0] {
                continue;
            }
            if (s[r - 1][c - 1] == 's' && s[r0 - 1][c0 - 1] == 'n')
                || (s[r - 1][c - 1] == 'n' && s[r0 - 1][c0 - 1] == 'u')
                || (s[r - 1][c - 1] == 'u' && s[r0 - 1][c0 - 1] == 'k')
                || (s[r - 1][c - 1] == 'k' && s[r0 - 1][c0 - 1] == 'e')
                || (s[r - 1][c - 1] == 'e' && s[r0 - 1][c0 - 1] == 's')
            {
                m[r0][c0] = true;
                queue.push_back((r0, c0));
            }
        }
    }

    m[h][w]
}

fn main() {
    input! {
        h: usize,
        _: usize,
        s: [Chars; h],
    }
    let yes = f(&s);
    println!("{}", if yes { "Yes" } else { "No" });
}
