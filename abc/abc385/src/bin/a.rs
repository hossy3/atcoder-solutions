use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let yes = (a == b && b == c) || (a + b == c) || (b + c == a) || (c + a == b);
    println!("{}", if yes { "Yes" } else { "No" });
}
