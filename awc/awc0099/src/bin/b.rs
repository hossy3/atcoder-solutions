use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        sr: [(usize, usize); m],
        t: [usize; n],
    }

    let mut rs: Vec<(usize, usize)> = vec![];
    for &(s, r) in sr.iter().sorted_unstable_by_key(|&&(s, r)| (r, s)) {
        if rs.is_empty() || rs.last().unwrap().1 < s {
            rs.push((r, s));
        }
    }

    let mut result = 0;
    for t in t {
        let i = rs.partition_point(|&(r, _)| r <= t);
        if i > 0 {
            result += rs[i - 1].1;
        }
    }
    println!("{result}");
}
