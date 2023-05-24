use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut count = 0;
    for i in 0..(n - 2) {
        let (x0, y0) = xy[i];
        for i in (i + 1)..(n - 1) {
            let (x1, y1) = xy[i];
            for i in (i + 1)..n {
                let (x2, y2) = xy[i];
                let (x01, y01) = (x1 - x0, y1 - y0);
                let (x02, y02) = (x2 - x0, y2 - y0);
                let linear = x01 * y02 - x02 * y01 == 0;
                if !linear {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}
