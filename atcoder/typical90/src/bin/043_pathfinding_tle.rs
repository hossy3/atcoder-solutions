use pathfinding::prelude::dijkstra_all;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        (h, w): (usize, usize),
        (rs, cs): (Usize1, Usize1),
        (rt, ct): (Usize1, Usize1),
        s: [Chars; h],
    }

    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let reachables = dijkstra_all(&(rs, cs, 0), |&(r, c, dir)| {
        let turn_cost = if r == rs && c == cs { 0 } else { 1 };
        let mut v: Vec<_> = (0..4)
            .filter(|&dir0| dir0 != dir)
            .map(|dir0| ((r, c, dir0), turn_cost))
            .collect();
        let (dr, dc) = dirs[dir];
        let r0 = r.wrapping_add_signed(dr);
        let c0 = c.wrapping_add_signed(dc);
        if r0 < h && c0 < w && s[r0][c0] != '#' {
            v.push(((r0, c0, dir), 0));
        }
        v
    });

    let result = (0..4)
        .map(|dir| reachables.get(&(rt, ct, dir)).unwrap().1)
        .min()
        .unwrap();
    println!("{result}");
}
