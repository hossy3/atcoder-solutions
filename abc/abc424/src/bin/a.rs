use proconio::input;

fn main() {
    input! {
        (a, b, c): (usize, usize, usize),
    }
    let yes = a == b || b == c || c == a;
    println!("{}", if yes { "Yes" } else { "No" });
}
