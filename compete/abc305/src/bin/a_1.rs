use proconio::input;

fn main() {
    input! { n: usize };
    let m = ((n + 2) / 5) * 5;
    println!("{}", m);
}
