use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut yp: [(usize, usize); n],
        l: [usize; q],
    }

    yp.sort();

    let mut cum_p = vec![0; n + 1];
    for i in 0..n {
        cum_p[i + 1] = cum_p[i] + yp[i].1;
    }
    for l in l {
        let i = yp.partition_point(|&(y, _)| y < l);
        let result = cum_p[n] - cum_p[i];
        println!("{result}");
    }
}
