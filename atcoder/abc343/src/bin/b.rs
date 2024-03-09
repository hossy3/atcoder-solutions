use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
    }
    for a in a {
        let v: Vec<_> = a
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| i + 1)
            .collect();
        let result = v.iter().join(" ");
        println!("{result}");
    }
}
