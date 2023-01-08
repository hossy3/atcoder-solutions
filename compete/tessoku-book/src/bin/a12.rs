use proconio::input;

// bin-search

fn f(t: usize, a: &[usize]) -> usize {
    let mut count = 0;
    for &x in a {
        count += t / x;
    }
    count
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut l = 1;
    let mut r = 1_000_000_000;
    while l != r {
        let m = (l + r) / 2;
        if f(m, &a) >= k {
            r = m; 
        } else {
            l = m + 1;
        }
    }
    println!("{}", l);
}
