use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }
    let yes = c < a || (c == a && d < b);
    println!("{}", if yes { "Yes" } else { "No" });
}
