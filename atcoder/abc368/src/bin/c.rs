use proconio::input;

fn main() {
    input! {
        n: usize,
        mut h: [i64; n],
    }
    let mut t = 0;
    let mut i = 0;
    while i < n {
        if (t + 1) % 3 == 0 {
            let j = h[i] / 5;
            t += j * 3;
            h[i] -= j * 5;

            if h[i] > 0 {
                t += 1;
                h[i] -= 3;
            }
        } else {
            t += 1;
            h[i] -= 1;
        }
        if h[i] <= 0 {
            i += 1;
        }
    }
    println!("{t}");
}
