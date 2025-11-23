use proconio::input;

fn f(n: usize) -> bool {
    let a = n / 100;
    let b = (n / 10) % 10;
    let c = n % 10;
    a * b == c
}

fn main() {
    input! {
        n: usize,
    }
    for i in n..=919 {
        if f(i) {
            println!("{i}");
            return;
        }
    }
}
