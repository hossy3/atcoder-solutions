use proconio::input;

fn main() {
    input! {
        n: usize,
        k: isize,
        a: [isize; n],
    }

    let mut cum = vec![0isize];
    for i in 0..n {
        cum.push(cum[i] + a[i]);
    }

    let mut map = std::collections::HashMap::new();
    for i in 0..=n {
        map.entry(cum[i]).or_insert(vec![]).push(i);
    }

    let mut result = 0usize;
    for (k0, v0) in &map {
        if let Some(v1) = map.get(&(k0 + k)) {
            for &i in v0 {
                let p = v1.partition_point(|&j| j <= i);
                result += v1.len() - p;
            }
        }
    }
    println!("{result}");
}
