use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        a: [usize; n]
    }
    let mut count = 0usize;
    for i0 in 0..(n - 4) {
        for i1 in (i0 + 1)..(n - 3) {
            let x = (a[i0] * a[i1]) % p;
            for i2 in (i1 + 1)..(n - 2) {
                let x = (x * a[i2]) % p;
                for i3 in (i2 + 1)..(n - 1) {
                    let x = (x * a[i3]) % p;
                    for i4 in (i3 + 1)..n {
                        let x = (x * a[i4]) % p;
                        if x == q {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", count);
}
