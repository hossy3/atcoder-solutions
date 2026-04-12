use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let yes = s.len() % 5 == 0;
    println!("{}", if yes { "Yes" } else { "No" });
}
