use proconio::input;

fn div_num(n: usize) -> usize {
    let mut count = 1usize;
    let mut n = n;
    let mut x = 2;
    while x * x <= n {
        if n % x == 0 {
            count += 1;
            n /= x;
        } else {
            x += if x == 2 { 1 } else { 2 };
        }
    }
    count
}

fn main() {
    input! {
        n: usize,
    }
    let m = div_num(n);
    let m = (m as f64).log2().ceil();
    println!("{m}");
}
