use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut v = vec![0];
    for x in a {
        let x = (v.last().unwrap() + x) % 360;
        v.push(x);
    }
    v.sort();
    v.push(360);
    let result = v.windows(2).map(|x| x[1] - x[0]).max().unwrap();
    println!("{}", result);
}
