use proconio::input;

fn main() {
    input! {
        s: String,
    }
    if s.ends_with("s") {
        println!("{s}es");
    } else {
        println!("{s}s");
    }
}
