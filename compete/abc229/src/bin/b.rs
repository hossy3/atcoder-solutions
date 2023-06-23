use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
    }
    let mut yes = false;
    while a > 0 && b > 0 {
        yes = yes || ((a % 10) + (b % 10) >= 10);
        a /= 10;
        b /= 10;
    }
    println!("{}", if yes { "Hard" } else { "Easy" });
}
