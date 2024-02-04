use proconio::input;

fn main() {
    input! {
        n: usize,
        mut dcs: [(usize, usize, usize); n],
    }
    dcs.sort();

    const D: usize = 5000;
    let mut v = vec![0usize; D + 1];
    v[0] = 0;

    for &(d, c, s) in &dcs {
        let prev = v.clone();
        for i in c..=d {
            v[i] = v[i].max(prev[i - c] + s);
        }
    }

    let result = v.iter().max().unwrap();
    println!("{result}");
}
