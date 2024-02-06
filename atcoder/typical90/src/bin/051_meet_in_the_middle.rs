use proconio::input;

trait BitTest {
    fn bit_test(&self, i: usize) -> bool;
}

impl BitTest for usize {
    fn bit_test(&self, i: usize) -> bool {
        self & (1 << i) != 0
    }
}

fn f(k: usize, a: &[usize]) -> Vec<Vec<usize>> {
    let n = a.len();
    let mut v = vec![vec![0; 0]; k + 1];
    for i in 0..((1 << n) as usize) {
        let m = i.count_ones() as usize;
        if m > k {
            continue;
        }
        let x = (0..n).filter(|&j| i.bit_test(j)).map(|j| a[j]).sum();
        v[m].push(x);
    }
    for i in 1..=k {
        v[i].sort();
    }
    v
}

fn main() {
    input! {
        n: usize,
        k: usize,
        p: usize,
        mut a: [usize; n],
    }
    let v0 = f(k, &a[..(n / 2)]);
    let v1 = f(k, &a[(n / 2)..]);
    let mut count = 0;
    for i in 0..=k {
        for &x in &v1[i] {
            let j = v0[k - i].partition_point(|&y| x + y <= p);
            count += j;
        }
    }
    println!("{count}");
}
