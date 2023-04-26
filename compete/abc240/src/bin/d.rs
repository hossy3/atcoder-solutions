use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut v = vec![]; // val, num
    for x in a {
        if v.is_empty() {
            v.push((x, 1));
        } else {
            let (x0, n0) = v[v.len() - 1];
            if x != x0 {
                v.push((x, 1));
            } else {
                if x == n0 + 1 {
                    v.resize(v.len() - n0, (0, 0));
                } else {
                    v.push((x, n0 + 1));
                }
            }
        }
        println!("{}", v.len());
    }
}
