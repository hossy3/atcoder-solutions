use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }
    let mut result = 0usize;
    while n > 0 {
        if n < 1_000 {
            n = 0;
        } else if n < 1_000_000 {
            result += n - 1_000 + 1;
            n = 1_000 - 1;
        } else if n < 1_000_000_000 {
            result += (n - 1_000_000 + 1) * 2;
            n = 1_000_000 - 1;
        } else if n < 1_000_000_000_000 {
            result += (n - 1_000_000_000 + 1) * 3;
            n = 1_000_000_000 - 1;
        } else if n < 1_000_000_000_000_000 {
            result += (n - 1_000_000_000_000 + 1) * 4;
            n = 1_000_000_000_000 - 1;
        } else {
            result += (n - 1_000_000_000_000_000 + 1) * 5;
            n = 1_000_000_000_000_000 - 1;
        }
    }

    println!("{result}");
}
