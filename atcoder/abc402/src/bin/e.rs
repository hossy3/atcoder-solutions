use proconio::input;

trait BitTest {
    fn bit_test(&self, i: usize) -> bool;
}

impl BitTest for usize {
    fn bit_test(&self, i: usize) -> bool {
        self & (1 << i) != 0
    }
}

fn main() {
    input! {
        n: usize,
        x: usize,
        scp: [(f64, usize, usize); n],
    }

    let mut d = vec![vec![0.0f64; x + 1]; 1 << n];
    for x in 0..=x {
        for s in 0..(1usize << n) {
            for (i, &(score, cost, p)) in scp.iter().enumerate() {
                if x < cost || s.bit_test(i) {
                    continue;
                }
                let x0 = x - cost;
                let s0 = s | (1 << i);
                let p0 = p as f64 / 100.0;
                let val = p0 * (d[s0][x0] + score) + (1.0 - p0) * d[s][x0];
                d[s][x] = d[s][x].max(val);
            }
        }
    }

    let result = d[0][x];
    println!("{result}");
}
