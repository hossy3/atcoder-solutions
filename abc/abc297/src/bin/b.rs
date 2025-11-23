use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let b0 = s.iter().position(|&x| x == 'B').unwrap();
    let b1 = s.iter().rposition(|&x| x == 'B').unwrap();
    let r0 = s.iter().position(|&x| x == 'R').unwrap();
    let r1 = s.iter().rposition(|&x| x == 'R').unwrap();
    let k = s.iter().rposition(|&x| x == 'K').unwrap();
    let yes = ((b0 + b1) % 2 == 1) && (r0 < k && k < r1);
    println!("{}", if yes { "Yes" } else { "No" });
}
