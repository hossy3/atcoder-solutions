use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut l: [usize; n],
    }
    l.sort();
    let result = l
        .iter()
        .combinations(3)
        .filter(|v| {
            let (l0, l1, l2) = (*v[0], *v[1], *v[2]);
            (l0 != l1 && l1 != l2) && (l0 + l1 > l2)
        })
        .count();
    println!("{result}");
}
