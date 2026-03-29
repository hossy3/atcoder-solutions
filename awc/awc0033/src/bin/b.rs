use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        l: isize,
        r: isize,
        t: [isize; n],
    }

    let v = t
        .iter()
        .map(|&t| {
            if t < l {
                l - t
            } else if t > r {
                t - r
            } else {
                0
            }
        })
        .sorted()
        .collect::<Vec<_>>();
    let result = v[..k].iter().sum::<isize>();
    println!("{result}");
}
