use pathfinding::directed::dijkstra::dijkstra_partial;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn pack(r: usize, c: usize, dir: usize) -> usize {
    debug_assert!(r < 1024);
    debug_assert!(c < 1024);
    debug_assert!(dir < 4);
    (r << 12) + (c << 2) + dir
}

fn unpack(node: usize) -> (usize, usize, usize) {
    debug_assert!(node < 1024 * 1024 * 4);
    (node >> 12, (node >> 2) % 1024, node % 4)
}

fn main() {
    input! {
        (h, w): (usize, usize),
        (rs, cs): (Usize1, Usize1),
        (rt, ct): (Usize1, Usize1),
        s: [Chars; h],
    }

    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let (reachables, t) = dijkstra_partial(
        &(pack(rs, cs, 0)),
        |&node| {
            let (mut r, mut c, dir) = unpack(node);
            let mut v = vec![];
            if r == rs && c == cs && dir == 0 {
                v.push((pack(r, c, 1), 0));
                v.push((pack(r, c, 2), 0));
                v.push((pack(r, c, 3), 0));
            } else if dir == 2 || dir == 3 {
                v.push((pack(r, c, 0), 1));
                v.push((pack(r, c, 1), 1));
            } else {
                v.push((pack(r, c, 2), 1));
                v.push((pack(r, c, 3), 1));
            }

            let (dr, dc) = dirs[dir];
            r = r.wrapping_add_signed(dr);
            c = c.wrapping_add_signed(dc);
            while r < h && c < w && s[r][c] != '#' {
                v.push((pack(r, c, dir), 0));
                r = r.wrapping_add_signed(dr);
                c = c.wrapping_add_signed(dc);
            }

            v
        },
        |&node| {
            let (r, c, _) = unpack(node);
            r == rt && c == ct
        },
    );

    let (_, result) = reachables.get(&t.unwrap()).unwrap();
    println!("{result}");
}
