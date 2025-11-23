use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut a = [0usize; 26];
    for c in s {
        let i = c as usize - 'a' as usize;
        a[i] += 1;
    }

    let mut counts = [0usize; 101];
    for x in a {
        counts[x] += 1;
    }

    let yes = counts[1..].iter().all(|&x| x == 0 || x == 2);
    println!("{}", if yes { "Yes" } else { "No" });
}
