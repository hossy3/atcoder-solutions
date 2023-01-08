use proconio::{input, marker::Usize1};

fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(Usize1, Usize1); n],
    }

    let mut imos = vec![0i64; d + 1];
    for &(l, r) in &lr {
        imos[l] += 1;
        imos[r + 1] -= 1;
    }

    let mut attendee = 0;
    for i in 0..d {
        attendee += imos[i];
        println!("{}", attendee);
    }
}
