use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let mut m = vec![vec![0; w]; h];
    m[0][0] = 1;
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    while let Some((i, j)) = queue.pop_front() {
        for k in 0..2 {
            let (i0, j0) = if k == 0 { (i + 1, j) } else { (i, j + 1) };
            if i0 < h && j0 < w {
                if c[i0][j0] == '.' && m[i0][j0] == 0 {
                    m[i0][j0] = m[i][j] + 1;
                    queue.push_back((i0, j0));
                }
            }
        }
    }
    let result = m.iter().map(|v| v.iter().max().unwrap()).max().unwrap();
    println!("{}", result);
}
