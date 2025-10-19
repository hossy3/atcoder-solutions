use std::collections::HashSet;

use proconio::{input, marker::Chars};

const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    input! {
        h: usize,
        w: usize,
        mut s: [Chars; h],
    }

    let mut set = HashSet::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            let mut count = 0;
            for &(di, dj) in &DIRS {
                let i = i.wrapping_add_signed(di);
                let j = j.wrapping_add_signed(dj);
                if i < h && j < w && s[i][j] == '#' {
                    count += 1;
                }
            }
            if count == 1 {
                set.insert((i, j));
            }
        }
    }

    while set.len() > 0 {
        for &(i, j) in &set {
            s[i][j] = '#';
        }

        let mut set0 = HashSet::new();
        for (i, j) in set {
            for &(di, dj) in &DIRS {
                let i = i.wrapping_add_signed(di);
                let j = j.wrapping_add_signed(dj);
                if !(i < h && j < w && s[i][j] == '.') {
                    continue;
                }

                let mut count = 0;
                for &(di, dj) in &DIRS {
                    let i = i.wrapping_add_signed(di);
                    let j = j.wrapping_add_signed(dj);
                    if i < h && j < w && s[i][j] == '#' {
                        count += 1;
                    }
                }
                if count == 1 {
                    set0.insert((i, j));
                }
            }
        }
        set = set0;
    }

    let result: usize = s
        .iter()
        .map(|s| s.iter().filter(|&&c| c == '#').count())
        .sum();
    println!("{result}");
}
