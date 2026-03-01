use proconio::input;

type Mint = ac_library::ModInt1000000007;

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
        a: [usize; n],
    }

    let mut result = Mint::new(0);
    for i in 0..60 {
        let mut x0 = 0; // 各ビットが 0
        let mut x1 = 0;
        for &x in &a {
            if x.bit_test(i) {
                x1 += 1;
            } else {
                x0 += 1;
            }
        }
        result += Mint::new(2).pow(i as u64) * x0 * x1;
    }

    println!("{result}");
}
