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
    let reachables = dijkstra_all(&(rs, cs, 0), |&(mut r, mut c, dir)| {
        let mut v = vec![];
        if r == rs && c == cs && dir == 0 {
            v.push(((r, c, 1), 0));
            v.push(((r, c, 2), 0));
            v.push(((r, c, 3), 0));
        } else if dir == 2 || dir == 3 {
            v.push(((r, c, 0), 1));
            v.push(((r, c, 1), 1));
        } else {
            v.push(((r, c, 2), 1));
            v.push(((r, c, 3), 1));
        }

        let (dr, dc) = dirs[dir];
        r = r.wrapping_add_signed(dr);
        c = c.wrapping_add_signed(dc);
        while r < h && c < w && s[r][c] != '#' {
            v.push(((r, c, dir), 0));
            r = r.wrapping_add_signed(dr);
            c = c.wrapping_add_signed(dc);
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
