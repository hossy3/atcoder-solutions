use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }
    let yes = n.iter().filter(|&&c| c == '1').count() == 1
        && n.iter().filter(|&&c| c == '2').count() == 2
        && n.iter().filter(|&&c| c == '3').count() == 3;
    println!("{}", if yes { "Yes" } else { "No" });
}
