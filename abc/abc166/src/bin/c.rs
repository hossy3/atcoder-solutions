use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        h: [usize; n],
        ab: [(Usize1, Usize1); m],
    }
    let mut v = vec![true; n];
    for (a, b) in ab {
        if !(h[a] > h[b]) {
            v[a] = false;
        }
        if !(h[b] > h[a]) {
            v[b] = false;
        }
    }
    let result = v.iter().filter(|&&b| b).count();
    println!("{result}");
}
