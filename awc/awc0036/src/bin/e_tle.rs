use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[isize; w]; h],
    }

    let mut result = isize::MIN;
    for i_bits in 0..(1 << h) {
        for j_bits in 0..(1 << w) {
            let mut sum = 0isize;
            for i in 0..h {
                for j in 0..w {
                    if (i_bits >> i) & 1 != 0 || (j_bits >> j) & 1 != 0 {
                        sum += a[i][j];
                    }
                }
            }
            result = result.max(sum);
        }
    }
    println!("{result}");
}
