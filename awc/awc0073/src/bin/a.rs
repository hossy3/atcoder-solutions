use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        k: usize,
        m: usize,
        a: [usize; k],
        b: [usize; m],
    }

    let set: HashSet<usize> = b.into_iter().collect();
    let result = a.iter().filter(|&x| set.contains(x)).count();
    println!("{result}");
}
