use proconio::input;

fn main() {
    input! {
        n: usize,
        abc: [(usize, usize, usize); n],
    }

    for (a, b, c) in abc {
        let result = a.min(c).min((a + b + c) / 3);
        println!("{result}");
    }
}
