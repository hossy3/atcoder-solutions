use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [usize; n],
    }
    let sum: usize = t.iter().sum();
    let mut v = vec![false; sum + 1];
    v[0] = true;
    for &t in &t {
        for i in (0..=(sum - t)).rev() {
            v[i + t] |= v[i];
        }
    }
    let result = (((sum + 1) / 2)..).find(|&i| v[i]).unwrap();
    println!("{}", result);
}
