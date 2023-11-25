use proconio::input;

fn g(mut n: usize, rev: bool) -> usize {
    let mut v = vec![];
    while n > 0 {
        v.push(n % 10);
        n /= 10;
    }
    v.sort();
    if rev {
        v.reverse();
    }
    let mut result = 0;
    for x in v {
        result *= 10;
        result += x;
    }
    result
}

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let result = (0..k).fold(n, |acc, _| g(acc, true) - g(acc, false));
    println!("{result}");
}
