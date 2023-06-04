use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let result = if n < 1000 {
        n
    } else if n < 10000 {
        (n / 10) * 10
    } else if n < 100000 {
        (n / 100) * 100
    } else if n < 1000000 {
        (n / 1000) * 1000
    } else if n < 10000000 {
        (n / 10000) * 10000
    } else if n < 100000000 {
        (n / 100000) * 100000
    } else {
        (n / 1000000) * 1000000
    };
    println!("{}", result);
}
