use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut k: Usize1,
    }
    let mut a = vec![];
    for _ in 0..n {
        input! {
            l: usize,
            a0: [usize; l],
        }
        a.push(a0);
    }
    input! {
        c: [usize; n],
    }

    for (i, &c) in c.iter().enumerate() {
        let m = a[i].len() * c;
        if k < m {
            let result = a[i][k % a[i].len()];
            println!("{result}");
            break;
        }
        k -= m;
    }
}
