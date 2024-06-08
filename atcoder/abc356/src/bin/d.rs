use proconio::input;

type Mint = ac_library::ModInt998244353;

trait BitTest {
    fn bit_test(&self, i: usize) -> bool;
}

impl BitTest for usize {
    fn bit_test(&self, i: usize) -> bool {
        self & (1 << i) != 0
    }
}

fn f(n: usize, k: usize) -> Mint {
    let k0 = 1usize << (k as u32);
    let k1 = 1usize << (k as u32 + 1);
    let mut i = Mint::new((n + 1) / k1) * k0;
    if (n / k0) % 2 == 1 {
        i += (n + 1) % k0;
    }
    i 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(f(4, 0).val(), 2);
        assert_eq!(f(4, 1).val(), 2);
        assert_eq!(f(4, 2).val(), 1);

        assert_eq!(f(5, 0).val(), 3);
        assert_eq!(f(5, 1).val(), 2);
        assert_eq!(f(5, 2).val(), 2);

        assert_eq!(f(7, 0).val(), 4);
        assert_eq!(f(7, 1).val(), 4);
        assert_eq!(f(7, 2).val(), 4);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut result = Mint::new(0);
    for k in 0..60 {
        if m.bit_test(k) {
            result += f(n, k);
        }
    }
    println!("{result}");
}
