use proconio::input;

fn f(n: usize) -> usize {
    if n < 10 || (n < 100 && (n / 10 == n % 10)) {
        1
    } else {
        0
    }
}

fn main() {
    input! {
        n: usize,
        d: [usize; n],
    }
    let mut result = 0;
    for i in 0..n {
        if f(i + 1) == 0 {
            continue;
        }
        for j in 1..=d[i] {
            if (i + 1) % 10 == j % 10 {
                result += f(j);
            }
        }
    }
    println!("{result}");
}
