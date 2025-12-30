use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        a: [[usize; w]; h],
        b: [usize; n],
    }
    let mut v = vec![vec![false; 91]; h];
    for (i, a) in a.iter().enumerate() {
        for &x in a {
            v[i][x] = true;
        }
    }
    let mut results = vec![0; h];
    for x in b {
        for i in 0..h {
            if v[i][x] {
                results[i] += 1;
            }
        }
    }
    let Some(result) = results.iter().max() else {
        unreachable!()
    };
    println!("{result}");
}
