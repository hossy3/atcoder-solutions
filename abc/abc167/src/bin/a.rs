use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }
    let yes = t.starts_with(&s);
    println!("{}", if yes { "Yes" } else { "No" });
}
