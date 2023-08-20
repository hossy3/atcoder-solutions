use proconio::input;

fn f(n: usize, a: &[usize]) -> bool {
    let sum: usize = a.iter().sum();
    if sum % 10 != 0 {
        return false;
    }
    let target = sum / 10;

    let mut j = 1;
    let mut s = a[0];
    for i in 0..n {
        while s < target {
            s += a[j];
            j = (j + 1) % n;
        }
        if s == target {
            return true;
        }
        s -= a[i];
    }
    false
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let yes = f(n, &a);
    println!("{}", if yes { "Yes" } else { "No" });
}
