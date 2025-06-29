use proconio::input;

fn f(a: &[usize]) -> usize {
    let n = a.len() / 2;
    let mut cur = vec![0usize];
    for &x in &a[0..n] {
        let prev = cur;
        cur = vec![0usize; prev.len() + 1];
        for (i, &y) in prev.iter().enumerate() {
            if i % 2 != cur.len() % 2 {
                continue;
            }
            if i > 0 {
                cur[i - 1] = cur[i - 1].max(y);
            }
            cur[i + 1] = cur[i + 1].max(x + y);
        }
    }

    for &x in &a[n..] {
        let prev = cur;
        cur = vec![0usize; prev.len() - 1];
        for (i, &y) in prev.iter().enumerate() {
            if i % 2 != cur.len() % 2 {
                continue;
            }
            if i > 0 {
                cur[i - 1] = cur[i - 1].max(y);
            }
            if i + 1 < cur.len() {
                cur[i + 1] = cur[i + 1].max(x + y);
            }
        }
    }

    cur[0]
}

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            a: [usize; n * 2],
        }
        let result = f(&a);
        println!("{result}");
    }
}
