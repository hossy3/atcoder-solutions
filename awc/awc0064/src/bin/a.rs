use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut v: Vec<Vec<usize>> = vec![];
    for (i, &a) in a.iter().enumerate() {
        let m = v.len();
        if a < k {
            continue;
        }
        if m == 0 || v[m - 1][v[m - 1].len() - 1] + 1 != i {
            v.push(vec![i]);
        } else {
            v[m - 1].push(i);
        }
    }

    let result = v.len();
    println!("{result}");
}
