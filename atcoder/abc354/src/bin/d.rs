use proconio::input;

fn f(a: i64, b: i64) -> i64 {
    let a0 = a.div_euclid(4) * 4; // div
    let b0 = b.div_euclid(2) * 2;
    let a1 = a - a0; // mod
    let b1 = b - b0;

    let mut area = a0 * b;
    area += b0 / 2
        * match a1 {
            1 => 3,
            2 => 6,
            3 => 7,
            _ => 0,
        };
    if b1 == 1 {
        area += match a1 {
            1 => 2,
            2 => 3,
            3 => 3,
            _ => 0,
        };
    }

    area
}

fn main() {
    input! {
        (a, b, c, d): (i64, i64, i64, i64)
    }

    let dx = a.div_euclid(4) * 4;
    let dy = b.div_euclid(2) * 2;
    let result = f(c - dx, d - dy) + f(a - dx, b - dy) - (f(c - dx, b - dy) + f(a - dx, d - dy));
    println!("{result}");
}
