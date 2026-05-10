use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        x: Usize1,
    }
    let result = a[x];
    println!("{result}");
}
