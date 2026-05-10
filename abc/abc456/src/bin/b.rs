use proconio::input;

fn main() {
    input! {
        a: [[usize; 6]; 3],
    }

    let mut count = 0usize;
    for i0 in 0..6 {
        for i1 in 0..6 {
            for i2 in 0..6 {
                let mut v = [a[0][i0], a[1][i1], a[2][i2]];
                v.sort();
                if v == [4, 5, 6] {
                    count += 1;
                }
            }
        }
    }

    let result = count as f64 / 216.0;
    println!("{result}");
}
