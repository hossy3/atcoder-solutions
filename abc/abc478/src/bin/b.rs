use proconio::input;

fn main() {
    input! {
        n: usize,
        v: usize,
        w: [usize; n],
    }

    let mut result = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                if i + j + k + 3 > v {
                    continue;
                }
                result = result.max(w[i] + w[j] + w[k]);
            }
        }
    }
    println!("{result}");
}
