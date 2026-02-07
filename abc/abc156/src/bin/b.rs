use proconio::input;

fn main() {
    input! {
        mut n: usize,
        k: usize,
    }
    let mut result = 0;
    while n > 0 {
        result += 1;
        n /= k;
    }
    println!("{result}");
}
