use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut a0 = (0..n).collect::<Vec<_>>();
    while a0.len() > 1 {
        let mut a1 = vec![];
        let mut a2 = vec![];
        for &x in &a0 {
            a2.push(x);
            if a2.len() == k {
                a1.push(*a2.iter().max_by_key(|&&i| (a[i], Reverse(i))).unwrap());
                a2.clear();
            }
        }
        if a2.len() > 0 {
            a1.push(*a2.iter().max_by_key(|&&i| (a[i], Reverse(i))).unwrap());
            a2.clear();
        }
        a0 = a1;
    }
    let result = a0[0] + 1;
    println!("{result}");
}
