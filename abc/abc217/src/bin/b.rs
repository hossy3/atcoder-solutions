use proconio::input;

fn main() {
    input! {
        s: [String; 3],
    }
    for &s0 in &["ABC", "ARC", "AGC", "AHC"] {
        if !s.contains(&s0.to_string()) {
            println!("{}", s0);
        }
    }
}
