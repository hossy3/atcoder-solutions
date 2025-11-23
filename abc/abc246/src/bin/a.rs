use proconio::input;

fn main() {
    input! {
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
        x3: i64,
        y3: i64,
    }
    let x = if x2 == x3 {
        x1
    } else if x1 == x3 {
        x2
    } else {
        x3
    };
    let y = if y2 == y3 {
        y1
    } else if y1 == y3 {
        y2
    } else {
        y3
    };
    println!("{} {}", x, y);
}
