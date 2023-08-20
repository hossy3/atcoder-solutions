use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let yes = (a..=b).any(|x| 100 % x == 0);
    println!("{}", if yes { "Yes" } else { "No" });
}
