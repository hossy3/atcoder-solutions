use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            w: usize,
            c: [usize; n],
        }

        let mut cum = vec![0usize; n + 1];
        for i in 0..n {
            cum[i + 1] = cum[i] + c[i];
        }

        let mut cost = usize::MAX;
        for x in 0..(2 * w) {
            let mut cost0 = 0;
            let mut j = x;
            if j >= w {
                cost0 += cum[(j - w).min(n)] - cum[0];
            }
            while j < n {
                cost0 += cum[(j + w).min(n)] - cum[j];
                j += 2 * w;
            }
            cost = cost.min(cost0);
        }
        println!("{cost}");
    }
}
