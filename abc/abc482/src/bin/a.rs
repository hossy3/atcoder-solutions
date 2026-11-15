use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let yes = s.iter().filter(|&&c| c == 'o').count() % 2 == 0;
    println!("{}", if yes { "even" } else { "odd" });
}
