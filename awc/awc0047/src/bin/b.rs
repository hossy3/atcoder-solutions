use proconio::input;

fn main() {
    input! {
        n: usize,
        mut m: usize,
        w: [u8; n - 1],
    }

    for (i, &w) in w.iter().enumerate() {
        if w == 1 {
            if m == 0 {
                println!("{}", i + 1);
                return;
            }
            m -= 1;
        }
    }
    println!("{n}");
}
