use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort();
    let result = a[n - 1] + a[n - 2];
    println!("{result}");
}
