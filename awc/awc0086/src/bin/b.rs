use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }

    a.sort_unstable();
    let m = k / 2;
    let result = a[n - 1] * (k - m) + a[n - 2] * m;
    println!("{result}");
}
