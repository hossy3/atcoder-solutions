use std::collections::HashSet;

use proconio::{input, marker::Chars};

type Mint = ac_library::ModInt998244353;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut uf = ac_library::Dsu::new(h * w);
    for i in 0..(h - 1) {
        for j in 0..w {
            if s[i][j] == '#' && s[i + 1][j] == '#' {
                uf.merge(i * w + j, (i + 1) * w + j);
            }
        }
    }
    for i in 0..h {
        for j in 0..(w - 1) {
            if s[i][j] == '#' && s[i][j + 1] == '#' {
                uf.merge(i * w + j, i * w + j + 1);
            }
        }
    }

    let mut red_num = 0;
    let mut green_groups = HashSet::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                green_groups.insert(uf.leader(i * w + j));
            } else {
                red_num += 1;
            }
        }
    }
    let green_groups_num = green_groups.len();

    let mut sum = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            let mut set = HashSet::new();
            if i > 0 && s[i - 1][j] == '#' {
                set.insert(uf.leader((i - 1) * w + j));
            }
            if i < h - 1 && s[i + 1][j] == '#' {
                set.insert(uf.leader((i + 1) * w + j));
            }
            if j > 0 && s[i][j - 1] == '#' {
                set.insert(uf.leader(i * w + j - 1));
            }
            if j < w - 1 && s[i][j + 1] == '#' {
                set.insert(uf.leader(i * w + j + 1));
            }
            sum += green_groups_num + 1 - set.len();
        }
    }

    let result = Mint::new(sum) / Mint::new(red_num);
    println!("{result}");
}
