use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut result = 0usize;
    for i in 1..=n {
        let j = (n - 1) / i;
        result += j;
    }
    println!("{result}");
}
