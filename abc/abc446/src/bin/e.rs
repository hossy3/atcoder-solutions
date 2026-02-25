use ac_library::Dsu;
use proconio::input;

fn main() {
    input! {
        m: usize,
        a: usize,
        b: usize,
    }

    let mut dsu = Dsu::new(m * m + 1);
    for x in 0..m {
        for y in 0..m {
            let z = x * m + y;
            if x == 0 || y == 0 {
                dsu.merge(z, m * m);
                continue;
            }

            let (x0, y0) = (y, (a * y + b * x) % m);
            let z0 = x0 * m + y0;
            dsu.merge(z, z0);
            if y0 == 0 {
                dsu.merge(z0, m * m);
            }
        }
    }

    let mut result = 0usize;
    for x in 0..m {
        for y in 0..m {
            if !dsu.same(x * m + y, m * m) {
                result += 1;
            }
        }
    }

    println!("{result}");
}
