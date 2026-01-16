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
        m: usize,
        x: usize,
        ca: [(usize, [usize; m]); n],
    }

    let mut result = usize::MAX;
    for i in 0..(1 << n) {
        let mut v = vec![0; m]; // 理解度
        let mut cost = 0;
        for (j, (c, a)) in ca.iter().enumerate() {
            if i.bit_test(j) {
                cost += c;
                for (k, x) in a.iter().enumerate() {
                    v[k] += x;
                }
            }
        }

        if v.iter().all(|&x0| x0 >= x) {
            result = result.min(cost);
        }
    }

    if result < usize::MAX {
        println!("{result}");
    } else {
        println!("-1");
    }
}
