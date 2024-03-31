use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); m],
    }
    let mut v = vec![0usize; n];
    for (a, b) in ab {
        if a < b {
            v[b] += 1;
        }
        if b < a {
            v[a] += 1;
        }
    }
    let result = v.iter().filter(|&&x| x == 1).count();
    println!("{result}");
}
