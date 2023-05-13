use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let t = s.iter().filter(|&&x| x == 'T').count();
    let a = n - t;
    let yes = t > a || (t == a && s[n - 1] == 'A');
    println!("{}", if yes { "T" } else { "A" });
}
