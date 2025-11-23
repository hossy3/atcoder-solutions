use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u32,
        a: [u128; n],
    }

    let max = 10_u128.pow(k);
    let mut result = 1_u128;
    for &x in &a {
        result *= x;
        if result >= max {
            result = 1;
        }
    }
    println!("{result}");
}
