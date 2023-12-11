use proconio::input;

fn main() {
    input! {
        k: usize,
        g: usize,
        m: usize,
    }
    let mut glass = 0;
    let mut mug = 0;
    for _ in 0..k {
        if glass == g {
            glass = 0;
        } else if mug == 0 {
            mug = m;
        } else {
            let x = (g - glass).min(mug);
            glass += x;
            mug -= x;
        }
    }
    println!("{glass} {mug}");
}
