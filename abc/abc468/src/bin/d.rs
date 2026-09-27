use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();
    let mut count = 0usize;

    // 長さ奇数の回文
    for i in 0..n {
        count += 1;
        let mut diff = 0usize;
        for j in 1..=(i.min(n - i - 1)) {
            // eprintln!("{}, {}, {}, {}", i, j, s[i - j], s[i + j]);
            if s[i - j] != s[i + j] {
                diff += 1;
                if diff > 1 {
                    break;
                }
            }
            count += 1;
            // eprintln!("i: {}, j: {}, count: {}", i, j, count);
        }
    }
    // eprintln!("{count}");

    // 長さ偶数の回文
    for i in 0..(n - 1) {
        let mut diff = 0usize;
        // eprintln!("b: {}", i.min(n - i - 2));
        for j in 0..=(i.min(n - i - 2)) {
            if s[i - j] != s[i + j + 1] {
                diff += 1;
                if diff > 1 {
                    break;
                }
            }
            count += 1;
        }
    }

    println!("{count}");
}
