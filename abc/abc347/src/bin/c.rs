use std::collections::BTreeSet;

use proconio::input;

fn f(a: usize, b: usize, d: &[usize]) -> bool {
    let n = d.len();
    if n == 1 {
        return true;
    }

    let mut set = BTreeSet::new();

    for (i, &x) in d.iter().enumerate() {
        set.insert((x % (a + b), i));
    }

    for _ in 0..n {
        let (min, i) = set.pop_first().unwrap();
        let max = set.last().unwrap().0;
        if max - min < a {
            return true;
        }
        set.insert((min + a + b, i));
    }

    false
}

fn main() {
    input! {
        (n, a, b): (usize, usize, usize),
        d: [usize; n],
    }

    let yes = f(a, b, &d);
    println!("{}", if yes { "Yes" } else { "No" });
}
