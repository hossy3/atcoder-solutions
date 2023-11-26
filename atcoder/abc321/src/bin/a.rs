use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }
    while n >= 10 {
        if (n / 10) % 10 <= n % 10 {
            println!("No");
            return;
        }
        n /= 10;
    }
    println!("Yes");
}
