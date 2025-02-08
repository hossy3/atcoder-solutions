use ac_library::convolution_i64;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
    }

    let Some(max) = s.iter().max() else {
        unreachable!()
    };
    let mut p = vec![0i64; max + 1];
    for &x in &s {
        p[x] = 1;
    }
    let v = convolution_i64(&p, &p);
    let result: i64 = s.iter().map(|&x| (v[x * 2] - 1) / 2).sum();
    println!("{result}");
}
