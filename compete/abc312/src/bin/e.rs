use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        b: [(usize, usize, usize, usize, usize, usize); n],
    }

    let mut m = vec![vec![vec![std::usize::MAX; 100]; 100]; 100];
    for (i, &(x0, y0, z0, x1, y1, z1)) in b.iter().enumerate() {
        for x in x0..x1 {
            for y in y0..y1 {
                for z in z0..z1 {
                    m[x][y][z] = i;
                }
            }
        }
    }

    let mut v = vec![HashSet::new(); n];
    let mut f = |xyz0: (usize, usize, usize), xyz1: (usize, usize, usize)| {
        let (x0, y0, z0) = xyz0;
        let (x1, y1, z1) = xyz1;
        if m[x0][y0][z0] != std::usize::MAX
            && m[x1][y1][z1] != std::usize::MAX
            && m[x0][y0][z0] != m[x1][y1][z1]
        {
            v[m[x0][y0][z0]].insert(m[x1][y1][z1]);
            v[m[x1][y1][z1]].insert(m[x0][y0][z0]);
        }
    };

    for x in 0..99 {
        for y in 0..100 {
            for z in 0..100 {
                f((x, y, z), (x + 1, y, z));
            }
        }
    }

    for x in 0..100 {
        for y in 0..99 {
            for z in 0..100 {
                f((x, y, z), (x, y + 1, z));
            }
        }
    }

    for x in 0..100 {
        for y in 0..100 {
            for z in 0..99 {
                f((x, y, z), (x, y, z + 1));
            }
        }
    }

    for set in v {
        let result = set.len();
        println!("{}", result);
    }
}
