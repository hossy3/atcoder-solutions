use std::collections::{HashMap, HashSet};

use ac_library::Dsu;
use proconio::{input, marker::Chars};

fn is_wall(s: &[Vec<char>], r: usize, c: usize) -> bool {
    s[r][c] == '#'
}

// 壁に隣り合っているか
fn is_adjacent_wall(s: &[Vec<char>], r: usize, c: usize) -> bool {
    let (h, w) = (s.len(), s[0].len());
    if r > 0 && s[r - 1][c] == '#' {
        return true;
    }
    if r < h - 1 && s[r + 1][c] == '#' {
        return true;
    }
    if c > 0 && s[r][c - 1] == '#' {
        return true;
    }
    if c < w - 1 && s[r][c + 1] == '#' {
        return true;
    }
    false
}

fn main() {
    input! {
        (h, w): (usize, usize),
        s: [Chars; h],
    }

    let f = |r: usize, c: usize| r * w + c;

    let mut dsu = Dsu::new(h * w);
    for r in 0..h {
        for c in 0..w {
            if is_wall(&s, r, c) || is_adjacent_wall(&s, r, c) {
                continue;
            }
            if r > 0 && !is_wall(&s, r - 1, c) && !is_adjacent_wall(&s, r - 1, c) {
                dsu.merge(f(r, c), f(r - 1, c));
            }
            if r < h - 1 && !is_wall(&s, r + 1, c) && !is_adjacent_wall(&s, r + 1, c) {
                dsu.merge(f(r, c), f(r + 1, c));
            }
            if c > 0 && !is_wall(&s, r, c - 1) && !is_adjacent_wall(&s, r, c - 1) {
                dsu.merge(f(r, c), f(r, c - 1));
            }
            if c < w - 1 && !is_wall(&s, r, c + 1) && !is_adjacent_wall(&s, r, c + 1) {
                dsu.merge(f(r, c), f(r, c + 1));
            }
        }
    }

    let mut map = HashMap::new();
    for r in 0..h {
        for c in 0..w {
            if is_wall(&s, r, c) {
                continue;
            }
            let i = dsu.leader(f(r, c));
            map.entry(i).or_insert(HashSet::new()).insert(f(r, c));
            if is_adjacent_wall(&s, r, c) {
                continue;
            }
            if r > 0 && !is_wall(&s, r - 1, c) {
                map.entry(i).or_insert(HashSet::new()).insert(f(r - 1, c));
            }
            if r < h - 1 && !is_wall(&s, r + 1, c) {
                map.entry(i).or_insert(HashSet::new()).insert(f(r + 1, c));
            }
            if c > 0 && !is_wall(&s, r, c - 1) {
                map.entry(i).or_insert(HashSet::new()).insert(f(r, c - 1));
            }
            if c < w - 1 && !is_wall(&s, r, c + 1) {
                map.entry(i).or_insert(HashSet::new()).insert(f(r, c + 1));
            }
        }
    }

    let result = map.iter().map(|(_, set)| set.len()).max().unwrap();
    println!("{result}");
}
