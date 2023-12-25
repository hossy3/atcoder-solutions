use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }
    let mut result = 0;
    for c in n {
        result = (result + (c as usize - '0' as usize)) % 9;
    }
    let yes = result == 0;
    println!("{}", if yes { "Yes" } else { "No" });
}
