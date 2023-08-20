use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut v = vec![0usize; 200];
    for &a in &a {
        v[a % 200] += 1;
    }
    let result: usize = v
        .iter()
        .map(|&x| if x > 1 { x * (x - 1) / 2 } else { 0 })
        .sum();
    println!("{}", result);
}
