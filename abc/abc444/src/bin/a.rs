use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let a = (n / 100) % 10;
    let b = (n / 10) % 10;
    let c = n % 10;

    let yes = a == b && b == c;
    println!("{}", if yes { "Yes" } else { "No" });
}
