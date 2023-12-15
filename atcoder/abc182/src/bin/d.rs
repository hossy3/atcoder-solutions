use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut result = 0;
    let mut cur = 0;
    let mut acc_all = 0;
    let mut acc_max = 0;
    for &x in &a {
        acc_all += x;
        acc_max = acc_max.max(acc_all);
        result = result.max(cur + acc_max);
        cur += acc_all;
    }
    println!("{result}");
}
