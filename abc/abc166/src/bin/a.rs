use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let result = if s.as_str() == "ABC" { "ARC" } else { "ABC" };
    println!("{result}");
}
