use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }
    let mut b = false;
    for c in &s {
        let mut c = *c;
        match c {
            '"' => {
                b = !b;
            }
            ',' => {
                if !b {
                    c = '.';
                }
            }
            _ => {}
        }
        print!("{}", c);
    }
    println!();
}
