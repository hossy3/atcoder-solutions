use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: Usize1,
    }
    for k in (0..10).rev() {
        let x = if (n & (1 << k)) > 0 { 7 } else { 4 };
        print!("{}", x);
    }
    println!();
}
