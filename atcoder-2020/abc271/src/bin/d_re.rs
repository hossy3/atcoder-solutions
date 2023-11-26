use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        ab: [(usize, usize); n],
    }

    if n == 1 {
        if ab[0].0 == s {
            println!("Yes");
            println!("H");
        } else if ab[0].1 == s {
            println!("Yes");
            println!("T");
        } else {
            println!("No");
        }
        return;
    }

    let m = n / 2;
    let mut v0 = vec![0; 1 << m];
    for i in 0..(1 << m) {
        for j in 0..m {
            v0[i] += if ((1 << j) & i) > 0 { ab[j].0 } else { ab[j].1 };
        }
    }

    let mut v1 = vec![0; 1 << (n - m)];
    for i in 0..(1 << (n - m)) {
        for j in 0..(n - m) {
            v1[i] += if ((1 << j) & i) > 0 {
                ab[m + j].0
            } else {
                ab[m + j].1
            };
        }
    }

    let mut set = BTreeSet::new();
    for &x in &v1 {
        set.insert(x);
    }

    for i in 0..(1 << m) {
        if v0[i] > s {
            continue;
        }
        if !set.contains(&(s - v0[i])) {
            continue;
        }
        for j in 0..(1 << (n - m)) {
            if v0[i] + v1[j] == s {
                println!("Yes");
                for k in 0..m {
                    let c = if ((1 << k) & i) > 0 { 'H' } else { 'T' };
                    print!("{}", c);
                }
                for k in 0..(n - m) {
                    let c = if ((1 << k) & j) > 0 { 'H' } else { 'T' };
                    print!("{}", c);
                }
                return;
            }
        }
    }
    println!("No");
}
