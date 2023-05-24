use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Usize1};

fn f(uv: &[(usize, usize)], p: &[usize]) -> i64 {
    let mut set = HashSet::new();
    let p0 = [0, 1, 2, 3, 4, 5, 6, 7];
    set.insert(p0);
    let mut queue = VecDeque::new();
    queue.push_back((0, p0, 8));
    while let Some((step, p0, p1)) = queue.pop_front() {
        if p0 == p {
            return step as i64;
        }
        let mut q = [8, 8, 8, 8, 8, 8, 8, 8, 8];
        for (i, &x) in p0.iter().enumerate() {
            q[x] = i;
        }
        for &(u, v) in uv {
            let j = if u == p1 {
                v
            } else if v == p1 {
                u
            } else {
                continue;
            };
            let mut p0 = p0.clone();
            p0[q[j]] = p1;
            if set.insert(p0) {
                queue.push_back((step + 1, p0, j));
            }
        }
    }
    -1
}

fn main() {
    input! {
        m: usize,
        uv: [(Usize1, Usize1); m],
        p: [Usize1; 8],
    }
    let result = f(&uv, &p);
    println!("{}", result);
}
