use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut v = vec![1usize; n + 1];
    v[0] = 0;
    let mut result = 1;
    for i in 2..=n {
        let jmax = n / i;
        for j in 1..=jmax {
            v[i * j] += 1;
        }
        result += i * v[i];
    }
    println!("{}", result);
}
