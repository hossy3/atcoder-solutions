use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: u32,
        a: [usize; 2usize.pow(n)],
    }
    let m = 2usize.pow(n - 1);
    let i0 = a[..m].iter().position_max().unwrap();
    let i1 = a[m..].iter().position_max().unwrap() + m;
    let result = if a[i0] < a[i1] { i0 + 1 } else { i1 + 1 };
    println!("{result}");
}
