use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        t: usize,
        cs: [(Usize1, usize); m],
    }

    let mut v = vec![(0usize, 0usize); n];
    for (c, s) in cs {
        v[c].0 += s;
        v[c].1 += 1;
    }
    let result = v.iter().filter(|&&(a, b)| b > 0 && a < t * b).count();
    println!("{result}");
}
