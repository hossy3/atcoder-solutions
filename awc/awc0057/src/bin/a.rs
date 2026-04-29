use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    let mut b = a.clone();
    for i in 0..n {
        if i > 0 && i < n - 1 && a[i - 1] == a[i + 1] && a[i - 1] != a[i] {
            b[i] = 0;
        }
    }
    let result = b.iter().fold(x, |acc, &x| acc ^ x);
    println!("{result}");
}
