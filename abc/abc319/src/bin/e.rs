use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        pt: [(usize, usize); n - 1],
        q: [usize],
    }

    const N: usize = 840;
    let mut v = Vec::with_capacity(N);
    for i in 0..N {
        let mut time = i;
        for &(p, t) in &pt {
            if time % p > 0 {
                time += p - (time % p);
            }
            time += t;
        }
        time -= i;
        v.push(time);
    }

    for &q in &q {
        let result = q + x + v[(q + x) % N] + y;
        println!("{result}");
    }
}
