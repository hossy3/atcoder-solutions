use proconio::{input, marker::Usize1};

fn main() {
    input! {
        a: [Usize1; 4],
    }
    let mut v = [0; 4];
    for i in a {
        v[i] += 1;
    }
    let result: usize = v.iter().map(|x| x / 2).sum();
    println!("{result}");
}
