use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort_unstable();
    if a.len() == 1 || a[n - 1] > a[n - 2] {
        println!("{}", a[n - 1]);
    } else {
        println!("0");
    }
}
