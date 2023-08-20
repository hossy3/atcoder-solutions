use proconio::input;

fn main() {
    input! {
        v: usize,
        a: usize,
        b: usize,
        c: usize,
    }
    let x = v % (a + b + c);
    let result = if x < a {
        'F'
    } else if x < (a + b) {
        'M'
    } else {
        'T'
    };
    println!("{}", result);
}
