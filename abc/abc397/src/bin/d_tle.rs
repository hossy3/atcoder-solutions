use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut x = 1usize;
    let mut y = 1usize;
    while x * x + x * y + y * y <= n {
        if x == y {
            x += 1;
            continue;
        }
        let z = (x - y) * (x * x + x * y + y * y);
        if z == n {
            println!("{x} {y}");
            return;
        } else if z < n {
            x += 1;
        } else {
            y += 1;
        }
    }
    println!("-1");
}
