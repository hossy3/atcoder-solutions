use proconio::input;

fn f(a: &[usize]) -> usize {
    let n = a.len();
    let mut r = 0;
    for i in 0..n {
        r = r.max(i + a[i]);
        if r == i + 1 {
            return r;
        }
    }
    n
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let result = f(&a);
    println!("{result}");
}
