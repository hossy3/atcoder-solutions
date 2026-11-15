use proconio::input;

fn f(w: usize, a0: usize, b0: usize, ab: &[(usize, usize)]) -> usize {
    if ab.len() == 0 {
        return a0; // おいしさの総和
    }
    let (a1, b1) = ab[0];
    if b0 + b1 <= w {
        f(w, a0, b0, &ab[1..]).max(f(w, a0 + a1, b0 + b1, &ab[1..]))
    } else {
        f(w, a0, b0, &ab[1..])
    }
}

fn main() {
    input! {
        n: usize,
        w: usize,
        ab: [(usize, usize); n],
    }

    let result = f(w, 0, 0, &ab);
    println!("{result}");
}
