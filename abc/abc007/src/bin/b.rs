use proconio::input;

fn main() {
    input! {
        a: String,
    }
    if a.as_str() == "a" {
        println!("-1");
    } else {
        println!("a");
    }
}
