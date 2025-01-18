use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn find_start(s: &[Vec<char>]) -> (usize, usize) {
    for r in 0..s.len() {
        for c in 0..s[0].len() {
            if s[r][c] == 'S' {
                return (r, c);
            }
        }
    }
    unreachable!();
}

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut m = vec![vec![vec![-1; 2]; w]; h];
    let start = find_start(&s);
    let mut queue = VecDeque::new();
    queue.push_back((start.0, start.1, 0, 0)); // (r, c, dir, step)
    queue.push_back((start.0, start.1, 1, 0)); // (r, c, dir, step)

    while let Some((r, c, dir, step)) = queue.pop_front() {
        if r >= h || c >= w {
            continue;
        }
        if m[r][c][dir] != -1 || s[r][c] == '#' {
            continue;
        }
        if s[r][c] == 'G' {
            println!("{}", step);
            return;
        }

        m[r][c][dir] = step;
        let step = step + 1;
        let dir = 1 - dir;

        if dir == 0 {
            queue.push_back((r + 1, c, dir, step));
            queue.push_back((r.wrapping_sub(1), c, dir, step));
        } else {
            queue.push_back((r, c + 1, dir, step));
            queue.push_back((r, c.wrapping_sub(1), dir, step));
        }
    }

    println!("-1");
}
