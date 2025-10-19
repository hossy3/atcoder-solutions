use proconio::input;

fn main() {
    input! {
        n: usize,
        abc: [(usize, usize, usize); n],
    }

    for (mut a, mut b, mut c) in abc {
        if c < a {
            (a, c) = (c, a);
        }
        if b < c {
            b = (b + c) / 2;
        }
        let result = a.min(b);
        println!("{result}");
    }
}
