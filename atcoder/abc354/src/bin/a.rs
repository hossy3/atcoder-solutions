use proconio::input;

fn main() {
    input! {
        h: usize,
    }
    let mut sum: usize = 0;
    for k in 0..32 {
        sum += 2usize.pow(k);
        if sum > h {
            println!("{}", k + 1);
            return;
        }
    }
}
