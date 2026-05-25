use proconio::input;

fn main() {
    input! {
        n: usize,
        lt: [(usize, usize); n],
    }

    for &(l, t) in &lt {
        let mut result = l / t;
        let r = l % t;
        if r >= t / 2 && t / 2 > 0 {
            result += 1;
        }
        println!("{result}");
    }
}
