use proconio::input;

fn f(n: usize, a: &[usize]) -> bool {
    let sum: usize = a.iter().sum();
    if sum % 10 != 0 {
        return false;
    }
    let target = sum / 10;

    let mut cum = vec![0usize; n * 2 + 1];
    for i in 0..(2 * n) {
        cum[i + 1] = cum[i] + a[i % n];
    }

    let yes = (0..n).any(|i| {
        let x = cum[i] + target;
        cum.binary_search(&x).is_ok()
    });
    yes
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let yes = f(n, &a);
    println!("{}", if yes { "Yes" } else { "No" });
}
