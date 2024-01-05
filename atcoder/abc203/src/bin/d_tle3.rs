use std::collections::BTreeSet;

use proconio::input;

fn f(
    set0: &mut BTreeSet<(usize, usize, usize)>,
    set1: &mut BTreeSet<(usize, usize, usize)>,
    rem: (usize, usize, usize),
    ins: (usize, usize, usize),
) {
    if set0.remove(&rem) {
        let set1_min = *set1.iter().next().unwrap();
        if ins < set1_min {
            set0.insert(ins);
        } else {
            set0.insert(set1_min);
            set1.remove(&set1_min);
            set1.insert(ins);
        }
    } else {
        let set0_max = *set0.iter().last().unwrap();
        if ins > set0_max {
            set1.insert(ins);
        } else {
            set1.insert(set0_max);
            set0.remove(&set0_max);
            set0.insert(ins);
        }
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [[usize; n]; n],
    }

    if k == 1 {
        let result = (0..n).map(|i| a[i].iter().min().unwrap()).min().unwrap();
        println!("{}", result);
        return;
    }

    let mut v = Vec::with_capacity(n * n);
    for i in 0..k {
        for j in 0..k {
            v.push((a[i][j], i, j));
        }
    }
    v.sort();

    let m = (k * k - 1) / 2;
    let mut set0 = BTreeSet::new();
    for &x in &v[..m] {
        set0.insert(x);
    }
    let mut set1 = BTreeSet::new();
    for &x in &v[m..] {
        set1.insert(x);
    }

    let mut result = std::usize::MAX;
    for i in 0..=(n - k) {
        let j0 = if i % 2 == 0 { 0 } else { n - k };
        if i != 0 {
            for j in j0..(j0 + k) {
                let rem = (a[i - 1][j], i - 1, j);
                let ins = (a[i + k - 1][j], i + k - 1, j);
                f(&mut set0, &mut set1, rem, ins);
            }
        }
        result = result.min(set1.iter().next().unwrap().0);

        if i % 2 == 0 {
            for j in 1..=(n - k) {
                for i in i..(i + k) {
                    let rem = (a[i][j - 1], i, j - 1);
                    let ins = (a[i][j + k - 1], i, j + k - 1);
                    f(&mut set0, &mut set1, rem, ins);
                }
                result = result.min(set1.iter().next().unwrap().0);
            }
        } else {
            for j in (1..=(n - k)).rev() {
                for i in i..(i + k) {
                    let rem = (a[i][j + k - 1], i, j + k - 1);
                    let ins = (a[i][j - 1], i, j - 1);
                    f(&mut set0, &mut set1, rem, ins);
                }
                result = result.min(set1.iter().next().unwrap().0);
            }
        }
    }

    println!("{}", result);
}
