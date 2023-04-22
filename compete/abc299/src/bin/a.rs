use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }
    let x0 = s.iter().position(|&x| x == '|');
    let x1 = s.iter().rposition(|&x| x == '|');
    let y = s.iter().position(|&x| x == '*');
    let yes = x0 < y && y < x1;
    println!("{}", if yes { "in" } else { "out" });
}
