use ac_library::{ModInt1000000007, ModInt998244353};
use itertools::Itertools;
use proconio::{input, marker::Chars};

type Mint0 = ModInt998244353;
type Mint1 = ModInt1000000007;

fn f(c: char) -> i32 {
    c as i32 - 'A' as i32
}

fn main() {
    input! {
        s: Chars,
    }
    let n = s.len();

    let mut v00 = vec![];
    let mut v01 = vec![];
    let mut p0 = Mint0::new(1);
    let mut p1 = Mint1::new(1);
    v00.push(Mint0::new(0));
    v01.push(Mint1::new(0));

    for &c in &s {
        let x0 = v00.last().unwrap() + p0 * f(c);
        let x1 = v01.last().unwrap() + p1 * f(c);
        v00.push(x0);
        v01.push(x1);
        p0 *= 26;
        p1 *= 26;
    }

    let mut v10 = vec![];
    let mut v11 = vec![];
    let mut p0 = Mint0::new(1);
    let mut p1 = Mint1::new(1);
    v10.push(Mint0::new(0));
    v11.push(Mint1::new(0));

    let s1: Vec<_> = s.iter().rev().collect();
    for &c in &s1 {
        let x0 = v10.last().unwrap() + p0 * f(*c);
        let x1 = v11.last().unwrap() + p1 * f(*c);
        v10.push(x0);
        v11.push(x1);
        p0 *= 26;
        p1 *= 26;
    }

    let mut i = n;
    while i > 0 {
        let k = n - i;
        if v00[n] - v00[k] == v10[i] * Mint0::new(26).pow(k as u64)
            && v01[n] - v01[k] == v11[i] * Mint1::new(26).pow(k as u64)
        {
            break;
        }
        i -= 1;
    }

    println!("{}{}", s.iter().join(""), s1[i..].iter().join(""));
}
