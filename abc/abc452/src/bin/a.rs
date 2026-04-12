use proconio::input;

fn main() {
    input! {
        m: usize,
        d: usize,
    }
    let yes = (m == 1 && d == 7)
        || (m == 3 && d == 3)
        || (m == 5 && d == 5)
        || (m == 7 && d == 7)
        || (m == 9 && d == 9);
    println!("{}", if yes { "Yes" } else { "No" });
}
