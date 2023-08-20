use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    if n == 0 {
        println!("{}", 0);
        return;
    }

    let mut result = 1usize << 60;
    let mut a = 0;
    let mut b = n.nth_root(3);
    if b * b * b < n {
        b += 1;
    }
    while a < b {
        while b > 0
            && (a * a * a + a * a * (b - 1) + a * (b - 1) * (b - 1) + (b - 1) * (b - 1) * (b - 1)
                >= n)
        {
            b -= 1;
        }
        result = result.min(a * a * a + a * a * b + a * b * b + b * b * b);
        a += 1;
    }
    println!("{}", result);
}
