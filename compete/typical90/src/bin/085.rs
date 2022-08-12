use std::collections::HashSet;

use proconio::input;

fn primes(n: usize) -> Vec<(usize, usize)> {
    let mut v = Vec::<(usize, usize)>::new();
    let mut n = n;
    let mut x = 2;
    while x * x <= n {
        if n % x == 0 {
            let mut k = 0;
            while n % x == 0 {
                k += 1;
                n /= x;
            }
            v.push((x, k));
        } else {
            x += if x == 2 { 1 } else { 2 };
        }
    }
    if n > 1 {
        v.push((n, 1));
    }
    v
}

fn f(
    base: &(usize, usize, usize),
    v: &[(usize, usize)],
    s: &mut HashSet<(usize, usize, usize)>,
) {
    if v.len() == 0 {
        s.insert(*base);
    } else {
        let (x, m) = v[0];
        for i in 0..=m {
            for j in 0..=(m - i) {
                let k = m - i - j;
                let mut b = [base.0 * x.pow(i as u32), base.1 * x.pow(j as u32), base.2 * x.pow(k as u32)];
                b.sort();
                f(&(b[0], b[1], b[2]), &v[1..], s);
            }
        }
    }
}

fn main() {
    input! {
        k: usize,
    }
    let v = primes(k);
    let mut s = HashSet::<(usize, usize, usize)>::new();
    f(&(1, 1, 1), &v, &mut s);
    println!("{}", s.len());
}
