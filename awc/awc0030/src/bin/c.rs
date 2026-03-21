use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [usize; n],
    }
    let mut v = vec![0usize; n + 1];
    for (i, &x) in s.iter().enumerate() {
        if x == 0 {
            v[i + 1] = 0;
        } else {
            v[i + 1] = v[i] + 1;
        }
    }
    let result = v.iter().filter(|&&x| x == k).count();
    println!("{result}");
}
