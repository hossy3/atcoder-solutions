use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        c: char,
        a: Chars,
    }

    let f = |c: char| match c {
        'R' => 1,
        'B' => 2,
        _ => 0,
    };

    let c0 = f(c);
    let a0 = a.iter().fold(0, |acc, &x| (acc + f(x)) % 3);
    let yes = c0 == a0;
    println!("{}", if yes { "Yes" } else { "No" });
}
