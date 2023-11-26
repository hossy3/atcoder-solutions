use proconio::input;

fn f(mut n: usize) -> bool {
    while n >= 10 {
        if (n / 10) % 10 <= n % 10 {
            return false;
        }
        n /= 10;
    }
    true
}

fn main() {
    input! {
        mut k: usize,
    }
    let mut i = 1;
    let mut count = 0;
    loop {
        if f(i) {
            count += 1;
            if count == k {
                println!("{i}");
                return;
            }
        }
        i += 1;

        if i % 1_000_000_000 == 0 {
            i = i.max(i / 10_000_000_000 * 10_000_000_000 + 9_000_000_000);
            i += 876_543_210;
        } else if i % 100_000_000 == 0 {
            i = i.max(i / 1_000_000_000 * 1_000_000_000 + 800_000_000);
            i += 76_543_210;
        } else if i % 10_000_000 == 0 {
            i = i.max(i / 100_000_000 * 100_000_000 + 70_000_000);
            i += 6_543_210;
        } else if i % 1_000_000 == 0 {
            i = i.max(i / 10_000_000 * 10_000_000 + 6_000_000);
            i += 543_210;
        } else if i % 100_000 == 0 {
            i = i.max(i / 1_000_000 * 1_000_000 + 500_000);
            i += 43_210;
        } else if i % 10_000 == 0 {
            i = i.max(i / 100_000 * 100_000 + 40_000);
            i += 3_210;
        } else if i % 1_000 == 0 {
            i = i.max(i / 10_000 * 10_000 + 3_000);
            i += 210;
        } else if i % 100 == 0 {
            i = i.max(i / 1_000 * 1_000 + 200);
            i += 10;
        }
    }
}
