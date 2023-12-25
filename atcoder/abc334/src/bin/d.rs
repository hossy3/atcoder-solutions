use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        r: [usize; n],
        query: [usize; q],
    }
    let mut v = r.clone();
    v.push(0);
    v.sort();
    for i in 0..n {
        v[i + 1] += v[i];
    }
    for &x in &query {
        let result = match v.binary_search(&x) {
            Ok(x) => x,
            Err(x) => x - 1,
        };
        println!("{result}");
    }
}
