use proconio::input;

trait ModInt {
    const MOD: usize = 1_000_000_007;

    fn modmul(&self, r: usize) -> usize;
    fn modpow(&self, k: usize) -> usize;
}

impl ModInt for usize {
    fn modmul(&self, r: usize) -> usize {
        (self * r) % Self::MOD
    }

    fn modpow(&self, k: usize) -> usize {
        if k == 1 {
            *self
        } else {
            let k0 = k / 2;
            let x = self.modpow(k0);
            if k % 2 == 1 {
                self.modmul(x).modmul(x)
            } else {
                x.modmul(x)
            }
        }
    }
}

fn f(n: usize, k: usize) -> usize {
    if k == 1 {
        if n == 1 {
            1
        } else {
            0
        }
    } else {
        if n == 1 {
            k
        } else if n == 2 {
            k.modmul(k - 1)
        } else {
            k.modmul(k - 1).modmul((k - 2).modpow(n - 2))
        }
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let count = f(n, k);
    println!("{count}");
}
