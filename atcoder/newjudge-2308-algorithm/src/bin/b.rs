use proconio::input;

fn main() {
    input! {
        mut x: usize,
        k: u32,
    }
    for i in 0..k {
        let k = 10usize.pow(i);
        x /= k;
        let m = x % 10;
        x -= m;
        if m >= 5 {
            x += 10;
        }
        x *= k;
    }
    println!("{x}");
}
