use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
    }
    let mut a = vec![];
    for _ in 0..n {
        input! {
            l: usize,
            a0: [usize; l],
        }
        a.push(a0);
    }
    input! {
        x: Usize1,
        y: Usize1,
    }
    let result = a[x][y];
    println!("{result}");
}
