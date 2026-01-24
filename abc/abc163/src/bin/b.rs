use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }
    let sum = a.iter().sum::<usize>();
    if n < sum {
        println!("-1");
    } else {
        println!("{}", n - sum);
    }
}
