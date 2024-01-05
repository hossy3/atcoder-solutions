use proconio::input;

fn main() {
    input! {
        a1: usize,
        a2: usize,
        a3: usize,
    }
    let yes = (a1 + a2 == a3 * 2) || (a2 + a3 == a1 * 2) || (a3 + a1 == a2 * 2);
    println!("{}", if yes { "Yes" } else { "No" });
}
