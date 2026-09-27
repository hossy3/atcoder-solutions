use proconio::input;

fn f(m: usize, a: &[usize], b: &[usize], x: usize) -> usize {
    let n = a.len();
    let mut diff = x;
    let mut result = x;
    for i in 0..n - 1 {
        diff = (a[i] + a[i + 1] + diff + m - b[i]) % m;
        result += diff;
    }
    result
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; n - 1],
    }

    let result = f(m, &a, &b, 0).min(f(m, &a, &b, 1));
    println!("{result}");
}
