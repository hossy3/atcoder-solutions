use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    let result = if b >= a {
        b * n
    } else if b + b >= a + (b / 2) {
        b * (n - 1) + a // 最終日だけ a にする
    } else if n % 2 == 0 {
        (a + b / 2) * (n / 2 - 1) + (a + (a / 2).max(b))
    } else {
        (a + b / 2) * (n - 1) / 2 + a
    };
    println!("{result}");
}
