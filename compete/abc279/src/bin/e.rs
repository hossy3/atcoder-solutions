use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    let mut np_last = Vec::new(); // pos to (number - 1)
    let mut pn_last = Vec::new(); // (number - 1) to pos
    for i in 0..n {
        np_last.push(i);
        pn_last.push(i);
    }
    for i in 0..m {
        let j = a[i];
        pn_last.swap(j - 1, j);
        np_last.swap(pn_last[j - 1], pn_last[j]);
    }

    let mut pn = Vec::new();
    let mut np = Vec::new();
    for i in 0..n {
        pn.push(i);
        np.push(i);
    }
    let mut pn_next = pn.clone();
    let mut np_next = np.clone();
    for i in 0..m {
        let j = a[i];
        pn_next.swap(j - 1, j);
        np_next.swap(pn[j - 1], pn[j]);

        let p = np[0];
        let n = pn_next[p];
        let p = np_last[n];
        println!("{}", p + 1);

        pn.swap(j - 1, j);
        np.swap(pn[j - 1], pn[j]);
    }
}
