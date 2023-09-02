use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: usize,
    }

    let result = if n < m { 0 } else { (n - m) / p + 1 };
    println!("{result}");
}
