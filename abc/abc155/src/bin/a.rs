use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let yes = (a == b && b != c) || (b == c && c != a) || (c == a && a != b);
    println!("{}", if yes { "Yes" } else { "No" });
}
