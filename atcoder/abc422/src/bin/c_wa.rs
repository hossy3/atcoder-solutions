use proconio::input;

fn main() {
    input! {
        n: usize,
        abc: [(usize, usize, usize); n],
    }

    for (a, b, c) in abc {
        let d = a.min(c);
        let result = d.min(a + c - d * 2 + b);
        println!("{result}");
    }
}
