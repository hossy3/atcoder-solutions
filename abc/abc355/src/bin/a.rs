use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    if a == b {
        println!("{}", -1);
    } else {
        let result = 6 - (a + b);
        println!("{result}");
    }
}
