use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 10],
    }
    let mut a = 10;
    let mut b = 1;
    let mut c = 10;
    let mut d = 1;
    for i in 0..10 {
        for j in 0..10 {
            if s[i][j] == '#' {
                a = a.min(i + 1);
                b = b.max(i + 1);
                c = c.min(j + 1);
                d = d.max(j + 1);
            }
        }
    }
    println!("{} {}", a, b);
    println!("{} {}", c, d);
}
