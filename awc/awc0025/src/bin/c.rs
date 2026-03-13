use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut d: [usize; n],
    }

    d.sort();
    if n == m {
        println!("0");
    } else {
        println!("{}", d[n - m - 1]);
    }
}
