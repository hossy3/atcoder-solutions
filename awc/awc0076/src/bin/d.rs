use std::collections::VecDeque;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn enc(s: (usize, usize), pq: &[(usize, usize)]) -> usize {
    let mut val = s.0 << 3 | s.1;
    for (i, (p, q)) in pq.iter().enumerate() {
        val |= (p << 3 | q) << ((i + 1) * 6);
    }
    val
}

fn dec(mut val: usize) -> ((usize, usize), (usize, usize), (usize, usize)) {
    let s = ((val >> 3) & 7, val & 7);
    val >>= 6;
    let pq0 = ((val >> 3) & 7, val & 7);
    val >>= 6;
    let pq1 = ((val >> 3) & 7, val & 7);
    (s, pq0, pq1)
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        s: (Usize1, Usize1),
        g: (Usize1, Usize1),
        pq: [(Usize1, Usize1); k],
        c: [Chars; n],
    }

    let mut visited = vec![false; 1 << 18];
    let mut queue = VecDeque::new();
    queue.push_back((0, enc(s, &pq)));
    while let Some((d, x)) = queue.pop_front() {
        // eprintln!("{d}, {x:b}");

        if visited[x] {
            continue;
        }
        visited[x] = true;

        let (s, pq0, pq1) = dec(x);
        for i in 0..5 {
            let s = match i {
                0 => (s.0.wrapping_sub(1), s.1),
                1 => (s.0 + 1, s.1),
                2 => (s.0, s.1.wrapping_sub(1)),
                3 => (s.0, s.1 + 1),
                4 => (s.0, s.1),
                _ => unreachable!(),
            };
            if s.0 >= n || s.1 >= m {
                continue;
            }
            if c[s.0][s.1] == '#' {
                continue;
            }
            if (k >= 1 && s == pq0) || (k >= 2 && s == pq1) {
                continue;
            }
            let d = d + 1;
            if s == g {
                println!("{d}");
                return;
            }

            let pq0 = if k < 1 {
                pq0
            } else if pq0.0 > s.0 && c[pq0.0 - 1][pq0.1] != '#' {
                (pq0.0 - 1, pq0.1)
            } else if pq0.0 < s.0 && c[pq0.0 + 1][pq0.1] != '#' {
                (pq0.0 + 1, pq0.1)
            } else if pq0.1 > s.1 && c[pq0.0][pq0.1 - 1] != '#' {
                (pq0.0, pq0.1 - 1)
            } else if pq0.1 < s.1 && c[pq0.0][pq0.1 + 1] != '#' {
                (pq0.0, pq0.1 + 1)
            } else {
                pq0
            };

            let pq1 = if k < 2 {
                pq1
            } else if pq1.0 > s.0 && c[pq1.0 - 1][pq1.1] != '#' {
                (pq1.0 - 1, pq1.1)
            } else if pq1.0 < s.0 && c[pq1.0 + 1][pq1.1] != '#' {
                (pq1.0 + 1, pq1.1)
            } else if pq1.1 > s.1 && c[pq1.0][pq1.1 - 1] != '#' {
                (pq1.0, pq1.1 - 1)
            } else if pq1.1 < s.1 && c[pq1.0][pq1.1 + 1] != '#' {
                (pq1.0, pq1.1 + 1)
            } else {
                pq1
            };

            if (k >= 1 && s == pq0) || (k >= 2 && s == pq1) {
                continue;
            }

            queue.push_back((d, enc(s, &[pq0, pq1])));
        }
    }

    println!("-1");
}
