use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
        t: [Usize1; q],
    }
    let mut v = vec![true; n];
    for i in t {
        v[i] = !v[i];
    }
    let result = v.iter().filter(|&&x| x).count();
    println!("{result}");
}
