use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let yes = s
        .windows(2)
        .filter(|v| ('A' <= v[1] && v[1] <= 'Z') && !t.contains(&v[0]))
        .count()
        == 0;
    println!("{}", if yes { "Yes" } else { "No" });
}
