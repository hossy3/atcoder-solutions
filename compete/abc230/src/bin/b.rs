use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let a = vec!['o', 'x', 'x'];
    let yes = s.iter().enumerate().all(|(i, &c)| c == a[i % 3])
        || s.iter().enumerate().all(|(i, &c)| c == a[(i + 1) % 3])
        || s.iter().enumerate().all(|(i, &c)| c == a[(i + 2) % 3]);
    println!("{}", if yes { "Yes" } else { "No" });
}
