use proconio::input;

fn main() {
    input! {
        l: usize,
        n1: usize,
        n2: usize,
        vl1: [(usize, usize); n1],
        vl2: [(usize, usize); n2],
    }

    let mut j = 0; // position
    let mut j1 = 0;
    let mut j2 = 0;
    let mut i1 = 0; // index of vl
    let mut i2 = 0;
    let mut count = 0;
    while j < l {
        let jnext = (j1 + vl1[i1].1).min(j2 + vl2[i2].1);
        if vl1[i1].0 == vl2[i2].0 {
            count += jnext - j;
        }
        j = jnext;
        if j1 + vl1[i1].1 == j {
            j1 = j;
            i1 += 1;
        }
        if j2 + vl2[i2].1 == j {
            j2 = j;
            i2 += 1;
        }
    }
    println!("{}", count);
}
