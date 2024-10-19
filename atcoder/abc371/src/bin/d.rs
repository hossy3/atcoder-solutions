use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [i64; n],
        p: [usize; n],
        q: usize,
        lr: [(i64, i64); q],
    }

    let mut cum = vec![0; n + 1];
    for (i, &p) in p.iter().enumerate() {
        cum[i + 1] = cum[i] + p;
    }

    for (l, r) in lr {
        let l0 = match x.binary_search(&l) {
            Ok(i) | Err(i) => i,
        };
        let r0 = match x.binary_search(&r) {
            Ok(i) => i + 1,
            Err(i) => i,
        };
        let result = cum[r0] - cum[l0];
        println!("{result}");
    }
}
