use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); n],
    }
    let mut v = vec![0isize; m];
    for (a, b) in ab {
        v[b] += 1;
        v[a] -= 1;
    }
    for x in v {
        println!("{x}");
    }
}
