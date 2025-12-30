use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut v = vec![];
    for x in a {
        v.push(x);
        while v.len() >= 4 && (1..4).all(|i| v[v.len() - i - 1] == v[v.len() - 1]) {
            for _ in 0..4 {
                v.pop();
            }
        }
    }
    let result = v.len();
    println!("{result}");
}
