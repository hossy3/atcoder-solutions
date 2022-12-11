use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: [usize; n],
    }
    let sum: usize = a.iter().sum();
    let mut t = t % sum;
    for i in 0..n {
        let x = a[i];
        if t < x {
            println!("{} {}", i + 1, t);
            break;
        }
        t -= x;
    }
}
