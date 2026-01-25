use std::collections::VecDeque;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn shortest_2dmap(m: &[Vec<char>], k: usize, sxy: (usize, usize), gxy: (usize, usize)) -> isize {
    const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let h = m.len();
    let w = m[0].len();
    let mut steps = vec![vec![isize::MAX; w]; h];
    let mut queue = VecDeque::new();
    steps[sxy.0][sxy.1] = 0;
    queue.push_back((0, sxy));

    while let Some((step, xy)) = queue.pop_front() {
        if xy.0 == gxy.0 && xy.1 == gxy.1 {
            return step;
        }
        for (dx, dy) in DIRS {
            for k in 1..=k {
                let x = xy.0.wrapping_add_signed(k as isize * dx);
                let y = xy.1.wrapping_add_signed(k as isize * dy);
                if x >= h || y >= w {
                    break;
                }
                if m[x][y] == '@' {
                    break;
                }
                if steps[x][y] <= step {
                    break;
                }
                if steps[x][y] == isize::MAX {
                    steps[x][y] = step + 1;
                    queue.push_back((step + 1, (x, y)));
                }
            }
        }
    }

    -1
}

fn main() {
    input! {
        h: usize,
        _: usize,
        k: usize,
        x1: Usize1,
        y1: Usize1,
        x2: Usize1,
        y2: Usize1,
        c: [Chars; h],
    }

    let result = shortest_2dmap(&c, k, (x1, y1), (x2, y2));
    println!("{result}");
}
