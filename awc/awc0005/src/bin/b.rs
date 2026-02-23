use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        mut s: [usize; n],
        pv: [(Usize1, usize); m],
    }
    for &(p, v) in &pv {
        s[p] = v;
    }
    let result = s.iter().filter(|&&x| x < k).count();
    println!("{result}");
}
