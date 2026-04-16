use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        p: usize,
        q: usize,
        s: [usize; n],
    }
    for s in s {
        let result = if s <= l {
            (s * p) / 100
        } else {
            (l * p + (s - l) * q) / 100
        };
        println!("{result}");
    }
}
