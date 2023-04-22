use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        c: [usize; n],
        r: [usize; n],
    }

    let mut x = 0;
    for i in 0..n {
        if c[i] == t && (x == 0 || r[i] > r[x - 1]) {
            x = i + 1;
        }
    }
    if x == 0 {
        for i in 0..n {
            if c[i] == c[0] && (x == 0 || r[i] > r[x - 1]) {
                x = i + 1;
            }
        }
    }
    println!("{}", x);
}
