use proconio::input;

fn f(sum_all: usize, sum_cur: usize, k: &[usize]) -> usize {
    if k.len() == 0 {
        sum_cur.max(sum_all - sum_cur)
    } else {
        f(sum_all, sum_cur + k[0], &k[1..]).min(f(sum_all, sum_cur, &k[1..]))
    }
}

fn main() {
    input! {
        n: usize,
        k: [usize; n],
    }
    let sum_all: usize = k.iter().sum();
    let result = f(sum_all, 0, &k);
    println!("{result}");
}
