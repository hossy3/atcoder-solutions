use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut result = 0; // 何年目
    let mut sum = 0;
    while sum < k {
        sum += result + n;
        result += 1;
    }
    result -= 1;
    println!("{result}");
}
