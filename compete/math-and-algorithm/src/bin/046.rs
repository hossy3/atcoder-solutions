use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn shortest_2dmap(m: &mut [Vec<bool>], syx: &(usize, usize), gyx: &(usize, usize)) -> usize {
    let mut queue = BinaryHeap::new();
    m[syx.0][syx.1] = false;
    queue.push((Reverse(0), *syx));

    while let Some((Reverse(step), yx)) = queue.pop() {
        if yx.0 == gyx.0 && yx.1 == gyx.1 {
            return step;
        }
        let a = [
            (yx.0 - 1, yx.1),
            (yx.0 + 1, yx.1),
            (yx.0, yx.1 - 1),
            (yx.0, yx.1 + 1),
        ];
        for &yx in &a {
            if m[yx.0][yx.1] {
                m[yx.0][yx.1] = false;
                queue.push((Reverse(step + 1), yx))
            }
        }
    }
    panic!();
}

fn main() {
    input! {
        r: usize,
        cmax: usize,
        syx: (Usize1, Usize1),
        gyx: (Usize1, Usize1),
        c: [Chars; r],
    }

    let mut m = vec![vec![true; cmax]; r];
    for y in 0..r {
        for x in 0..cmax {
            m[y][x] = c[y][x] == '.';
        }
    }

    let result = shortest_2dmap(&mut m, &syx, &gyx);
    println!("{}", result);
}
