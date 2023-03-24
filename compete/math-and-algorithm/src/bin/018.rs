use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut v = [0usize; 4];
    for x in a {
        v[x / 100 - 1] += 1;
    }
    let result = v[0] * v[3] + v[1] * v[2];
    println!("{}", result);
}
