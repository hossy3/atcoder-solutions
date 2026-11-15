use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [usize; n],
    }

    for i in 0..n {
        let result = if i == 0 || i == n - 1 {
            (x[n - 1] - x[0]).div_ceil(2)
        } else {
            let t = (x[i] - x[i - 1]).div_ceil(2);
            let mut result = t
                + (((x[i] - t).saturating_sub(x[0] + t)).div_ceil(2))
                    .max((x[n - 1] - x[i]).div_ceil(2));
            // eprintln!("0: {t}, {result}");
            if (x[i] - x[i - 1]) % 2 == 1 {
                result = result.min(
                    t + (((x[i] - t + 1).saturating_sub(x[0] + t)).div_ceil(2))
                        .max((x[n - 1] - x[i] - 1).div_ceil(2)),
                );
                // eprintln!("1: {t}, {result}");
            }

            let t = (x[i + 1] - x[i]).div_ceil(2);
            result = result.min(
                t + (((x[n - 1] - t).saturating_sub(x[i] + t)).div_ceil(2))
                    .max((x[i] - x[0]).div_ceil(2)),
            );
            // eprintln!("2: {t}, {result}");
            if (x[i + 1] - x[i]) % 2 == 1 {
                result = result.min(
                    t + (((x[n - 1] - t).saturating_sub(x[i] + t - 1)).div_ceil(2))
                        .max((x[i] - x[0] - 1).div_ceil(2)),
                );
                // eprintln!("3: {t}, {result}");
            }
            result
        };

        println!("{result}");
    }
}
