use proconio::input;

fn f(mut n: usize) -> usize {
    let mut result = 0;
    while n > 0 {
        result += n % 10;
        n /= 10;
    }
    result
}

fn main() {
    input! {
        n: usize,
    }
    let mut a = vec![0; n + 1];
    a[0] = 1;
    for i in 1..=n {
        for j in 0..i {
            a[i] += f(a[j]);
        }
    }
    let result = a[n];
    println!("{result}");
}
