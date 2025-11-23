use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![];
    for i in 0..n {
        input! {
            a0: [Usize1; i + 1],
        }
        a.push(a0);
    }

    let mut result = 0;
    for i in 0..n {
        let (x, y) = (result.min(i), result.max(i));
        result = a[y][x];
    }
    println!("{}", result + 1);
}
