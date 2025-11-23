use proconio::input;

fn dist2((x0, y0): (i64, i64), (x1, y1): (i64, i64)) -> i64 {
    (x1 - x0).pow(2) + (y1 - y0).pow(2)
}

fn main() {
    input! {
        (xa, ya): (i64, i64),
        (xb, yb): (i64, i64),
        (xc, yc): (i64, i64),
    }
    let d0 = dist2((xa, ya), (xb, yb));
    let d1 = dist2((xb, yb), (xc, yc));
    let d2 = dist2((xc, yc), (xa, ya));
    let yes = (d0 + d1 == d2) || (d1 + d2 == d0) || (d2 + d0 == d1);
    println!("{}", if yes { "Yes" } else { "No" });
}
