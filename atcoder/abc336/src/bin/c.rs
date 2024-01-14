use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }

    n -= 1;
    let mut result = 0usize;
    let mut k = 0;
    while n > 0 {
        result += (n % 5) * 2 * 10usize.pow(k);
        n /= 5;
        k += 1;
    }
    println!("{result}");
}
