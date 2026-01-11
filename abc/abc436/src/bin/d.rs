use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Chars};

fn get_positions(a: &[Vec<char>], c: char) -> Vec<(usize, usize)> {
    let mut v = vec![];
    for (i, a) in a.iter().enumerate() {
        for (j, &x) in a.iter().enumerate() {
            if x == c {
                v.push((i, j));
            }
        }
    }
    v
}

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    let mut visited = vec![vec![-1; w]; h];
    let mut warped = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back((0, (0, 0)));
    visited[0][0] = 0;

    while let Some((count, (i, j))) = queue.pop_front() {
        if i == h - 1 && j == w - 1 {
            println!("{count}");
            return;
        }

        for &(di, dj) in &dirs {
            let i0 = i as i64 + di;
            let j0 = j as i64 + dj;
            if i0 < 0 || j0 < 0 {
                continue;
            }
            let i0 = i0 as usize;
            let j0 = j0 as usize;
            if i0 >= h || j0 >= w {
                continue;
            }

            let c = s[i0][j0];
            if c == '#' {
                continue;
            }
            if visited[i0][j0] != -1 {
                continue;
            }
            visited[i0][j0] = count;
            queue.push_back((count + 1, (i0, j0)));
        }

        let c = s[i][j];
        if 'a' <= c && c <= 'z' && warped.insert(c) {
            let v = get_positions(&s, c);
            for (i0, j0) in v {
                if visited[i0][j0] != -1 {
                    continue;
                }
                visited[i0][j0] = count;
                queue.push_back((count + 1, (i0, j0)));
            }
        }
    }

    println!("-1");
}
