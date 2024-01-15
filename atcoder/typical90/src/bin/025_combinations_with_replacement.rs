use itertools::Itertools;
use proconio::input;

fn f0(mut n: usize) -> [usize; 10] {
    let mut a = [0usize; 10];
    while n > 0 {
        a[n % 10] += 1;
        n /= 10;
    }
    a
}

fn f1(v: &[&usize]) -> [usize; 10] {
    let mut a = [0usize; 10];
    for &&i in v {
        a[i] += 1;
    }
    a
}

fn f(n: usize, b: usize) -> usize {
    let n_digits = n.ilog10() as usize + 1;
    let nums: Vec<_> = (0usize..=9).collect();
    let mut result = 0usize;
    for i in 1..=n_digits {
        for v in nums.iter().combinations_with_replacement(i) {
            let mul = v.iter().fold(1, |acc, &&x| acc * x);
            let x = mul + b;
            if x <= n && f0(x) == f1(&v) {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    input! {
        n: usize,
        b: usize,
    }
    let result = f(n, b);
    println!("{result}");
}
