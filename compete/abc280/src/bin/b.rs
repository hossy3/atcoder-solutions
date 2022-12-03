use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [i64; n],
    }
    print!("{}", s[0]);
    for i in 1..n {
        print!(" {}", s[i] - s[i - 1]);
    }
    println!();
}
