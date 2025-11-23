use std::collections::{BinaryHeap, HashMap};

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn get_pos(a: &[Vec<char>], c: char) -> (usize, usize) {
    for (i, a) in a.iter().enumerate() {
        for (j, &x) in a.iter().enumerate() {
            if x == c {
                return (i, j);
            }
        }
    }
    unreachable!()
}

fn f(a: &[Vec<char>], rce: &[(usize, usize, i64)]) -> bool {
    let h = a.len();
    let w = a[0].len();

    let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    let (sx, sy) = get_pos(&a, 'S');

    let mut map = HashMap::new();
    for &(r, c, e) in rce {
        map.insert((r, c), e);
    }

    let mut visited = vec![vec![-1; w]; h];

    let mut heap = BinaryHeap::new();
    heap.push((0, (sx, sy)));

    while let Some((mut e, (x, y))) = heap.pop() {
        if let Some(&e0) = map.get(&(y, x)) {
            e = e.max(e0);
        }
        if visited[y][x] >= e {
            continue;
        }
        visited[y][x] = e;
        if e <= 0 {
            continue;
        }

        for &(dx, dy) in &dirs {
            let x0 = x as i64 + dx;
            let y0 = y as i64 + dy;
            if x0 < 0 || y0 < 0 {
                continue;
            }
            let x0 = x0 as usize;
            let y0 = y0 as usize;
            if x0 >= w || y0 >= h {
                continue;
            }

            let c = a[y0][x0];
            if c == '#' {
                continue;
            }
            if c == 'T' {
                return true;
            }

            heap.push((e - 1, (x0, y0)));
        }
    }

    false
}

fn main() {
    input! {
        (h, _): (usize, usize),
        a: [Chars; h],
        n: usize,
        rce: [(Usize1, Usize1, i64); n],
    }
    let yes = f(&a, &rce);
    println!("{}", if yes { "Yes" } else { "No" });
}
