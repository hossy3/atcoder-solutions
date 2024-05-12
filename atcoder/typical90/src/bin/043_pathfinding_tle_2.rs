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
        let mut v = Vec::with_capacity(3);
        if r == rs && c == cs && dir == 0 {
            v.push(((r, c, 1), 0));
            v.push(((r, c, 2), 0));
            v.push(((r, c, 3), 0));
        }
        let (dr, dc) = dirs[dir];
        let r0 = r.wrapping_add_signed(dr);
        let c0 = c.wrapping_add_signed(dc);
        if r0 < h && c0 < w && s[r0][c0] != '#' {
            v.push(((r0, c0, dir), 0));
            if dir == 2 || dir == 3 {
                v.push(((r0, c0, 0), 1));
                v.push(((r0, c0, 1), 1));
            } else {
                v.push(((r0, c0, 2), 1));
                v.push(((r0, c0, 3), 1));
            }
        }
        v
    });

    let result = (0..4)
        .filter_map(|dir| reachables.get(&(rt, ct, dir)))
        .map(|x| x.1)
        .min()
        .unwrap();
    println!("{result}");
}
