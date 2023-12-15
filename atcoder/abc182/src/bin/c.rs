use proconio::input;

fn f(mut n: usize) -> i64 {
    let mut a = [0, 0, 0];
    while n > 0 {
        let i = (n % 10) % 3;
        a[i] += 1;
        n /= 10;
    }

    let sum: usize = a.iter().sum();
    let rem = (a[1] + 2 * a[2]) % 3;
    if rem % 3 == 0 {
        return 0;
    }
    if sum == 1 {
        return -1;
    }
    if a[rem % 3] > 0 {
        return 1;
    }
    if sum == 2 {
        return -1;
    }
    2
}

fn main() {
    input! {
        n: usize,
    }
    let result = f(n);
    println!("{result}");
}
