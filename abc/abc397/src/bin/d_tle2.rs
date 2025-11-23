use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut x = (n as f64).powf(1.0 / 3.0) as usize;
    let mut y = 1usize;
    let mut z = x * x + x * y + y * y;
    while z <= n {
        if x == y {
            x += 1;
            continue;
        }
        z = x * x + x * y + y * y;
        let result = (x - y) * z;
        if result == n {
            println!("{x} {y}");
            return;
        } else if result < n {
            x += 1;
        } else {
            y += 1;
        }
    }
    println!("-1");
}
