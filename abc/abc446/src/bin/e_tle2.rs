use fixedbitset::FixedBitSet;
use proconio::input;

fn main() {
    input! {
        m: usize,
        a: usize,
        b: usize,
    }

    let mut result = 0usize;
    for x in 0..m {
        for y in 0..m {
            let mut bitset = FixedBitSet::with_capacity(m * m);
            let mut x = x % m;
            let mut y = y % m;
            if x == 0 || y == 0 {
                continue;
            }
            bitset.set(x * m + y, true);

            loop {
                (x, y) = (y, (a * y + b * x) % m);
                if y == 0 {
                    break;
                }
                if bitset.contains(x * m + y) {
                    result += 1;
                    break;
                }
                bitset.set(x * m + y, true);
            }
        }
    }

    println!("{result}");
}
