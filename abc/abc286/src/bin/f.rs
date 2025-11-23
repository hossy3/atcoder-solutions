use itertools::Itertools;
use proconio::input_interactive;

fn build_a(a0: &[i64], m: usize) -> Vec<i64> {
    let mut v = Vec::with_capacity(m);
    let mut i = 0;
    for &x in a0 {
        for j in 0..x {
            v.push(i + ((j + 1) % x) + 1);
        }
        i += x;
    }
    v
}

fn build_r(a0: &[i64], b: &[i64]) -> Vec<i64> {
    let mut v = Vec::with_capacity(a0.len());
    let mut i = 0usize;
    for &x in a0 {
        v.push(b[i] - (i + 1) as i64);
        i += x as usize;
    }
    v
}

fn main() {
    let a0 = vec![4i64, 9, 5, 7, 11, 13, 17, 19, 23];
    let m = a0.iter().sum::<i64>() as usize;
    let a = build_a(&a0, m);

    println!("{m}");
    println!("{}", a.iter().join(" "));

    input_interactive! {
        b: [i64; m],
    }

    let r = build_r(&a0, &b);
    eprintln!("{:?}", r);
    let (y, _) = ac_library::crt(&r, &a0);
    println!("{y}");
}
