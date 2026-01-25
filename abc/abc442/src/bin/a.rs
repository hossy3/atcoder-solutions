use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let result = s.iter().filter(|&&c| c == 'i' || c == 'j').count();
    println!("{result}");
}
