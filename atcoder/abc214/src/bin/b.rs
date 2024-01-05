use proconio::input;

fn main() {
    input! {
        s: usize,
        t: usize,
    }
    let mut result = 0;
    for a in 0..=s {
        for b in 0..=(s - a) {
            for c in 0..=(s - a - b) {
                if a * b * c <= t {
                    result += 1;
                }
            }
        }
    }
    println!("{}", result);
}
