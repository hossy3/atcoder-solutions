use proconio::input;

fn main() {
    input! {
        l: usize,
    }
    let mut result = 1usize;
    for i in 1..=11 {
        result *= l - i;
        result /= i;
    }
    println!("{result}");
}
