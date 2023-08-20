use proconio::input;

fn main() {
    input! {
        mut a: i64,
        mut b: i64,
        mut c: i64,
    }
    if c % 2 == 0 {
        a = a.abs();
        b = b.abs();
    }
    let result = if a < b {
        '<'
    } else if a > b {
        '>'
    } else {
        '='
    };
    println!("{}", result);
}
