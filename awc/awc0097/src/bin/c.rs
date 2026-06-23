use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut results = vec![usize::MAX; n];
    for i in 0..n {
        if results[i] != usize::MAX {
            continue;
        }
        results[i] = 0;
        for j in (i + 1)..n {
            if a[j] >= a[j - 1] {
                break;
            }
            results[j] = j;
        }
    }
    println!("{}", results.iter().join(" "));
}
