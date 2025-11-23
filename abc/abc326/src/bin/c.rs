use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }
    a.sort();
    let result = (0..n)
        .map(|i| a.lower_bound(&(a[i] + m)) - i)
        .max()
        .unwrap();
    println!("{result}");
}
