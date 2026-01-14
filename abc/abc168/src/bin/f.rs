use std::collections::HashSet;

use itertools::Itertools;
use proconio::input;

fn f(abc: &[(isize, isize, isize)], def: &[(isize, isize, isize)]) -> usize {
    // 座標圧縮する
    let mut xs = HashSet::new(); // 縦 (x) の壁
    for &(a, b, _) in abc {
        xs.insert(a);
        xs.insert(b);
    }
    for &(d, _, _) in def {
        xs.insert(d);
    }

    let mut ys = HashSet::new(); // 横 (y) の壁
    for &(_, _, c) in abc {
        ys.insert(c);
    }
    for &(_, e, f) in def {
        ys.insert(e);
        ys.insert(f);
    }

    let xs = xs.into_iter().sorted().collect::<Vec<_>>();
    let ys = ys.into_iter().sorted().collect::<Vec<_>>();

    // 座標圧縮後の壁を組み立てる
    let mut vs = vec![vec![false; ys.len()]; xs.len()]; // 縦方向に仕切る壁
    for &(a, b, c) in abc {
        let x0 = xs.binary_search(&a).unwrap();
        let x1 = xs.binary_search(&b).unwrap();
        let y = ys.binary_search(&c).unwrap();
        for x in x0..x1 {
            vs[x][y] = true;
        }
    }

    let mut hs = vec![vec![false; ys.len()]; xs.len()]; // 横方向に仕切る壁
    for &(d, e, f) in def {
        let x = xs.binary_search(&d).unwrap();
        let y0 = ys.binary_search(&e).unwrap();
        let y1 = ys.binary_search(&f).unwrap();
        for y in y0..y1 {
            hs[x][y] = true;
        }
    }

    // 塗りつぶす
    let mut m = vec![vec![false; ys.len() + 1]; xs.len() + 1];
    let x0 = xs.partition_point(|&x| x < 0);
    let y0 = ys.partition_point(|&y| y < 0);
    m[x0][y0] = true;
    let mut stack = vec![(x0, y0)];
    let mut area = 0;
    while let Some((x, y)) = stack.pop() {
        if x == 0 || x == xs.len() || y == 0 || y == ys.len() {
            return usize::MAX;
        }
        area += ((xs[x] - xs[x - 1]) * (ys[y] - ys[y - 1])) as usize;

        if !m[x + 1][y] && !hs[x][y - 1] {
            let x = x + 1;
            m[x][y] = true;
            stack.push((x, y));
        }
        if !m[x - 1][y] && !hs[x - 1][y - 1] {
            let x = x - 1;
            m[x][y] = true;
            stack.push((x, y));
        }
        if !m[x][y + 1] && !vs[x - 1][y] {
            let y = y + 1;
            m[x][y] = true;
            stack.push((x, y));
        }
        if !m[x][y - 1] && !vs[x - 1][y - 1] {
            let y = y - 1;
            m[x][y] = true;
            stack.push((x, y));
        }
    }

    area
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(isize, isize, isize); n],
        def: [(isize, isize, isize); m],
    }

    let result = f(&abc, &def);
    if result < usize::MAX {
        println!("{result}");
    } else {
        println!("INF");
    }
}
