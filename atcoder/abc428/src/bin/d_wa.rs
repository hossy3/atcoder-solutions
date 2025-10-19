use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            c: usize,
            d: usize,
        }

        let mut v = vec![];
        let mut c0 = c + 1;
        let mut d0 = d - 1;
        loop {
            let c1 = 10usize.pow(c0.ilog10() + 1) - 1; // 999 などにする
            if c0 + d0 <= c1 {
                v.push((c0, c0 + d0));
                break;
            } else {
                v.push((c0, c1));
                d0 -= c1 + 1 - c0;
                c0 = c1 + 1;
            }
        }
        eprintln!("{:?}", &v);

        let mut result = 0;
        for (l, r) in v {
            let c = c * 10usize.pow(l.ilog10() + 1);
            let l0 = ((c + l) as f64).sqrt().ceil() as usize;
            let r0 = ((c + r) as f64).sqrt().floor() as usize;
            result += r0 + 1 - l0;
            eprintln!("{} {}", l0, r0);
        }
        println!("{result}");
    }
}
