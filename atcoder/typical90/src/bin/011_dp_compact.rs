use proconio::input;

fn main() {
    input! {
        n: usize,
        mut dcs: [(usize, usize, usize); n],
    }
    dcs.sort_by_key(|&(d, c, _)| d as i64 - c as i64);

    const D: usize = 5000;
    let mut v = vec![0usize; D + 1];
    v[0] = 0;

    for &(d, c, s) in &dcs {
        let prev = v.clone();
        for i in 0..=D {
            v[i] = v[i].max(prev[i]);
            let j = i + c;
            if j <= d {
                v[j] = v[j].max(prev[i] + s);
            }
        }
    }

    let result = v.iter().max().unwrap();
    println!("{result}");
}
