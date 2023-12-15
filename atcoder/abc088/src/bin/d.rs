use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn f(h: usize, w: usize, s: &[Vec<char>]) -> i64 {
    let mut m = vec![vec![false; w + 2]; h + 2]; // walkable
    let black: usize = s
        .iter()
        .map(|v| v.iter().filter(|&&c| c == '#').count())
        .sum();
    for i in 0..h {
        for j in 0..w {
            m[i + 1][j + 1] = s[i][j] == '.';
        }
    }

    let mut stack = VecDeque::new();
    stack.push_back((0usize, (1, 1)));
    while let Some((step, (i, j))) = stack.pop_front() {
        if !m[i][j] {
            continue;
        }
        m[i][j] = false;
        if (i, j) == (h, w) {
            return (h * w - (step + 1) - black) as i64;
        }
        stack.push_back((step + 1, (i + 1, j)));
        stack.push_back((step + 1, (i - 1, j)));
        stack.push_back((step + 1, (i, j + 1)));
        stack.push_back((step + 1, (i, j - 1)));
    }

    -1
}

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let result = f(h, w, &s);
    println!("{result}");
}
