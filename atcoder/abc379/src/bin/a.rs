use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let a = n / 100;
    let b = (n / 10) % 10;
    let c = n % 10;
    println!("{b}{c}{a} {c}{a}{b}");
}
