use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let result = if a + b >= 15 && b >= 8 {
        1
    } else if a + b >= 10 && b >= 3 {
        2
    } else if a + b >= 3 {
        3
    } else {
        4
    };
    println!("{}", result);
}
