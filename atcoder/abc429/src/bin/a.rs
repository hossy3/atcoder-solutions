use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    for i in 0..n {
        let result = if i + 1 <= m {
            "OK"
        } else {
            "Too Many Requests"
        };
        println!("{result}");
    }
}
