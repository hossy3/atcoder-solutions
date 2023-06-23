use proconio::input;

fn main() {
    input! {
        n: usize,
        mut w: usize,
        mut ab: [(usize, usize); n],
    }

    ab.sort();
    let mut result = 0;
    for &(a, b) in ab.iter().rev() {
        if w <= b {
            result += a * w;
            break;
        } else {
            result += a * b;
            w -= b;
        }
    }
    println!("{}", result);
}
