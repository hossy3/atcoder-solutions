use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[isize; w]; h],
    }

    let mut result = isize::MIN;
    for i_bits in 0..(1 << h) {
        let mut sum = 0isize;
        for j in 0..w {
            let mut sum_i = 0isize;
            let mut sum_j = 0isize;
            for i in 0..h {
                if (i_bits >> i) & 1 != 0 {
                    sum_i += a[i][j];
                } else {
                    sum_j += a[i][j];
                }
            }
            sum += sum_i + sum_j.max(0);
        }
        result = result.max(sum);
    }
    println!("{result}");
}
