use proconio::input;

fn main() {
    input! {
        h: usize,
        m: usize,
    }
    let mut k = h * 60 + m;
    loop {
        let a = (k / 600, (k / 60) % 10, (k % 60) / 10, k % 10);
        if a.0 * 10 + a.2 < 24 && a.1 * 10 + a.3 < 60 {
            println!("{} {}", a.0 * 10 + a.1, a.2 * 10 + a.3);
            return;
        }
        k = (k + 1) % (24 * 60);
    }
}
