use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn f(k: usize, s: &[Vec<char>], i: usize, j: usize, set: &mut HashSet<usize>) -> usize {
    let h = s.len();
    let w = s[0].len();

    if s[i][j] == '#' {
        return 0;
    }
    let key = i * w + j;
    if set.contains(&key) {
        return 0;
    }
    if set.len() == k {
        return 1;
    }

    set.insert(key);

    let mut result = 0;
    for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let i = i.wrapping_add_signed(di);
        let j = j.wrapping_add_signed(dj);
        if i >= h || j >= w {
            continue;
        }
        if s[i][j] == '#' {
            continue;
        }
        result += f(k, s, i, j, set);
    }

    set.remove(&key);

    result
}

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h],
    }

    let mut result = 0;
    let mut set = HashSet::new();
    for i in 0..h {
        for j in 0..w {
            result += f(k, &s, i, j, &mut set);
        }
    }
    println!("{result}");
}
