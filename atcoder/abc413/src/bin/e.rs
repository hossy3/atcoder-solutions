use itertools::Itertools;
use proconio::input;

fn f(a: Vec<usize>) -> Vec<usize> {
    let n = a.len();
    if n == 2 {
        if a[0] < a[1] {
            return a;
        } else {
            return vec![a[1], a[0]];
        }
    }

    let a0 = f(a[..(n / 2)].to_vec());
    let a1 = f(a[(n / 2)..].to_vec());
    if a0 < a1 {
        return [a0, a1].concat();
    } else {
        return [a1, a0].concat();
    }
}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            p: [usize; 1 << n],
        }
        let result = f(p);
        println!("{}", result.iter().join(" "));
    }
}
