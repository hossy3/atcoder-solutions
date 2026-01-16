use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
        _c: isize,
        k: isize,
    }
    let result = if k <= a { k } else { a - (k - a - b).max(0) };
    println!("{result}");
}
