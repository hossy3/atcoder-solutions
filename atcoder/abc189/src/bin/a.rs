use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        c: String,
    }
    let yes = c.chars().tuple_windows().all(|(x, y)| x == y);
    println!("{}", if yes { "Won" } else { "Lost" });
}
