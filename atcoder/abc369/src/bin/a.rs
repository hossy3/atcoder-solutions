use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let result = if a == b {
        1
    } else if (a + b) % 2 == 1 {
        2
    } else {
        3
    };
    println!("{result}");
}
