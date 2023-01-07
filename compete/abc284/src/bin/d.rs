use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
        }

        let mut i = 2;
        while n % i != 0 {
            i += if i == 2 { 1 } else { 2 };
        }

        if n % (i * i) == 0 {
            let p = i;
            let q = n / (i * i);
            println!("{} {}", p, q);
        } else {
            let p = (n / i).sqrt();
            let q = i;
            println!("{} {}", p, q);
        }
    }
}
