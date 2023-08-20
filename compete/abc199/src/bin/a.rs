use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let yes = a * a + b * b < c * c;
    println!("{}", if yes { "Yes" } else { "No" });
}
