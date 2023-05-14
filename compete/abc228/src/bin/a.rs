use proconio::input;

fn main() {
    input! {
        s: usize,
        t: usize,
        x: usize,
    }
    let yes = if s < t {
        s <= x && x < t
    } else {
        x < t || s <= x
    };
    println!("{}", if yes { "Yes" } else { "No" });
}
