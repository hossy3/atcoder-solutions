use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let yes = a != b && b == c;
    println!("{}", if yes { "Yes" } else { "No" });
}
