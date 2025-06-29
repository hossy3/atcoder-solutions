use std::cmp::Reverse;

use proconio::input;

fn f(h: usize, m: usize, ab: &[(usize, usize)]) -> usize {
    let n = ab.len();
    let mut cur = vec![(h, m)];
    for (i, &(h0, m0)) in ab.iter().enumerate() {
        let mut cand = vec![];
        for &(h1, m1) in &cur {
            if h1 >= h0 {
                cand.push((h1 - h0, m1));
            }
            if m1 >= m0 {
                cand.push((h1, m1 - m0));
            }
        }
        if cand.len() == 0 {
            return i;
        }

        cand.sort_by_key(|&(h, m)| (h, Reverse(m)));
        cur.clear();
        cur.push(cand[0]);
        for &hm in &cand[1..] {
            let (h1, m1) = hm;
            let (h2, m2) = cur[cur.len() - 1];
            if h1 > h2 {
                if m1 >= m2 {
                    cur.pop();
                }
                cur.push(hm);
            }
        }
    }

    n
}

fn main() {
    input! {
        n: usize,
        h: usize,
        m: usize,
        ab: [(usize, usize); n],
    }
    let result = f(h, m, &ab);
    println!("{result}");
}
