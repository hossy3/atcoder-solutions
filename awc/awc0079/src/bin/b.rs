use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        tp: [(usize, usize); k],
        c: [usize; n],
    }

    let c = c.iter().sum::<usize>();
    let i = tp.partition_point(|&(t, _)| t <= c) - 1;
    let result = c - c * tp[i].1 / 100;
    println!("{result}");
}
