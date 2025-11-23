use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    'outer: for i in 0..=n {
        for j in 1..=9 {
            if (n % j > 0) || (i % (n / j) > 0) {
                continue;
            }
            print!("{j}");
            continue 'outer;
        }
        print!("-");
    }
    println!();
}
