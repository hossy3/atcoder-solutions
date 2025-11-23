use std::{cell::RefCell, thread::LocalKey};

use ac_library::{ButterflyCache, Modulus};
use proconio::{input, marker::Chars};

/// Represents $1000000009$.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Mod1000000009 {}

impl Modulus for Mod1000000009 {
    const VALUE: u32 = 1_000_000_009;
    const HINT_VALUE_IS_PRIME: bool = true;

    fn butterfly_cache() -> &'static LocalKey<RefCell<Option<ButterflyCache<Self>>>> {
        thread_local! {
            static BUTTERFLY_CACHE: RefCell<Option<ButterflyCache<Mod1000000009>>> = RefCell::default();
        }
        &BUTTERFLY_CACHE
    }
}

type Mint0 = ac_library::ModInt998244353;
type Mint1 = ac_library::ModInt1000000007;
type Mint2 = ac_library::StaticModInt<Mod1000000009>;

fn build_rolling_hash(chars: &[char]) -> Vec<(Mint0, Mint1, Mint2)> {
    let mut v = Vec::with_capacity(chars.len() + 1);
    v.push((Mint0::new(0), Mint1::new(0), Mint2::new(0)));
    let mut k0 = Mint0::new(1);
    let mut k1 = Mint1::new(1);
    let mut k2 = Mint2::new(1);
    for &x in chars {
        let y = if x == '0' { 0 } else { 1 };
        let x = *v.last().unwrap();
        let x0 = x.0 + Mint0::new(y) * k0;
        let x1 = x.1 + Mint1::new(y) * k1;
        let x2 = x.2 + Mint2::new(y) * k2;
        v.push((x0, x1, x2));
        k0 *= 2;
        k1 *= 2;
        k2 *= 2;
    }
    v
}

fn main() {
    input! {
        t: usize,
    }

    'outer: for _ in 0..t {
        input! {
            a: Chars,
            b: Chars,
        }
        let n = a.len();
        let a_hash = build_rolling_hash(&[a.clone(), a].concat());
        let b_hash = build_rolling_hash(&b);
        for i in 0..n {
            if a_hash[i + n].0 - a_hash[i].0 == b_hash[n].0 * Mint0::new(2).pow(i as u64)
                && a_hash[i + n].1 - a_hash[i].1 == b_hash[n].1 * Mint1::new(2).pow(i as u64)
                && a_hash[i + n].2 - a_hash[i].2 == b_hash[n].2 * Mint2::new(2).pow(i as u64)
            {
                println!("{i}");
                continue 'outer;
            }
        }
        println!("-1");
    }
}
