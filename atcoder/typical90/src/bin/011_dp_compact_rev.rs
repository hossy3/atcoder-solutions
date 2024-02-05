use proconio::input;

fn main() {
    input! {
        n: usize,
        mut dcs: [(usize, usize, usize); n],
    }
    dcs.sort();
    dcs.reverse();

    const D: usize = 5000;
    let mut v = vec![0usize; D + 1];
    v[0] = 0;

    for &(d, c, s) in &dcs {
        if d < c {
            continue;
        }
        let prev = v.clone();
        for i in 0..=(d - c) {
            v[i] = v[i].max(prev[i + c] + s);
        }
    }

    let result = v.iter().max().unwrap();
    println!("{result}");
}
