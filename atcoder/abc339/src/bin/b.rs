use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
    }
    let mut m = vec![vec!['.'; w]; h];
    let dirs = [(h - 1, 0), (0, 1), (1, 0), (0, w - 1)]; // URDL
    let mut dir = 0;
    let mut i = 0;
    let mut j = 0;
    for _ in 0..n {
        if m[i][j] == '.' {
            m[i][j] = '#';
            dir = (dir + 1) % 4;
        } else {
            m[i][j] = '.';
            dir = (dir + 3) % 4;
        }
        let (di, dj) = dirs[dir];
        i = (i + h + di) % h;
        j = (j + w + dj) % w;
    }

    for v in m {
        println!("{}", v.iter().join(""));
    }
}
