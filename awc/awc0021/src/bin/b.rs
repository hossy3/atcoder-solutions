use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        p: [usize; m],
    }

    for _ in 0..n {
        input! {
            k: usize,
            mut c: [Usize1; k],
        }

        if k == 0 {
            println!("0");
            continue;
        }

        c.sort();

        let mut cur = 0;
        for i in 1..k {
            if p[c[i]] > p[c[cur]] {
                cur = i;
            }
        }

        println!("{}", c[cur] + 1);
    }
}
