use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let result = if a == b {
        c
    } else if b == c {
        a
    } else if c == a {
        b
    } else {
        0
    };
    println!("{}", result);
}
