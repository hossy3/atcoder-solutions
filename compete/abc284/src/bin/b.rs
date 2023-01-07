use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            a: [usize],
        }
        let mut count = 0;
        for &a in &a {
            if a % 2 == 1 {
                count += 1;
            }
        }
        println!("{}", count);
    }
}
