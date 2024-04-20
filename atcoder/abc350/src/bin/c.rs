use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut a: [Usize1; n],
    }

    let mut a_inv: Vec<usize> = vec![0; n]; // number to index
    for (i, &x) in a.iter().enumerate() {
        a_inv[x] = i;
    }

    let mut results = vec![];
    for i in 0..n {
        if a[i] != i {
            let j = a_inv[i];
            results.push((i + 1, j + 1));
            a.swap(i, j);
            a_inv.swap(a[i], a[j]);
        }
    }

    println!("{}", results.len());
    for (i, j) in results {
        println!("{i} {j}");
    }
}
