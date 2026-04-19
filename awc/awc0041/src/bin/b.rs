use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        w: [usize; m - 1],
        b: [usize; n],
    }

    let mut w0 = vec![w[0]]; // そのマスまでの一番大きな段差
    for i in 1..m - 1 {
        let x = w0[i - 1].max(w[i]);
        w0.push(x);
    }

    for b in b {
        let result = w0.partition_point(|&w| w <= b) + 1;
        println!("{result}");
    }
}
