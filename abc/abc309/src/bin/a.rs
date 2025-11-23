use proconio::{input, marker::Usize1};

fn main() {
    input! {
        a: Usize1,
        b: Usize1,
    }
    let yes = (a / 3) == (b / 3) && a + 1 == b;
    println!("{}", if yes { "Yes" } else { "No" });
}
