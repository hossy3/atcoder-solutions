use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        b: [isize; n],
    }

    let mut results: Vec<(isize, isize)> = vec![]; // (連続した値, ここまでの最大値)
    for b in b {
        if let Some(&(cur, max)) = results.last() {
            let cur = (cur + b).max(b);
            let max = max.max(cur);
            results.push((cur, max));
        } else {
            results.push((b, b));
        }
    }
    println!("{}", results.iter().map(|(_, max)| max).join("\n"));
}
