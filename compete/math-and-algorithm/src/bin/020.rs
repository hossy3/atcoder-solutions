use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut count = 0;
    for i0 in 0..(n - 4) {
        for i1 in (i0 + 1)..(n - 3) {
            for i2 in (i1 + 1)..(n - 2) {
                for i3 in (i2 + 1)..(n - 1) {
                    for i4 in (i3 + 1)..n {
                        if a[i0] + a[i1] + a[i2] + a[i3] + a[i4] == 1000 {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", count);
}
