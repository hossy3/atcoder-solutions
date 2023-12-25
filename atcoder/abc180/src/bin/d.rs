use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        a: usize,
        b: usize,
    }
    let mut str = x;
    let mut exp = 0usize;
    while str * (a - 1) <= b && str * a < y {
        str *= a;
        exp += 1;
    }
    exp += (y - str - 1) / b;
    println!("{exp}");
}
