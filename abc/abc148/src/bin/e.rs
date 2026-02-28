use proconio::input;

fn f(n: usize) -> usize {
    // すべて奇数の時は 0
    if n % 2 == 1 {
        return 0;
    }

    // すべて偶数の時は 5で割れる回数
    let mut x = n / 2;
    let mut result = 0;
    while x > 0 {
        x /= 5;
        result += x;
    }

    result
}

fn main() {
    input! {
        n: usize,
    }
    let result = f(n);
    println!("{result}");
}
