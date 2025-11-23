use itertools::Itertools;
use proconio::input;

fn f(v: Vec<usize>, k: usize, r: &[usize]) {
    if r.len() == 0 {
        if v.iter().sum::<usize>() % k == 0 {
            println!("{}", v.iter().join(" "));
        }
    } else {
        for i in 1..=r[0] {
            let mut v = v.clone();
            v.push(i);
            f(v, k, &r[1..]);
        }
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        r: [usize; n],
    }
    f(vec![], k, &r);
}
