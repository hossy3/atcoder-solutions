use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        mut x: usize,
    }
    let sum: usize = a.iter().sum();
    let base = (x / sum) * n;
    x = x % sum;
    for i in 0..n {
        if x < a[i] {
            let result = base + i + 1;
            println!("{}", result);
            return;
        }
        x -= a[i];
    }
    panic!();
}
