use proconio::input;

fn diff_count(mut i: usize, mut j: usize) -> usize {
    let mut count = 0;
    for _ in 0..4 {
        if i % 10 != j % 10 {
            count += 1;
        }
        i /= 10;
        j /= 10;
    }
    count
}

fn main() {
    input! {
        n: usize,
        st: [(usize, usize); n],
    }

    const MAX: usize = 10000;

    let mut v = [true; MAX];
    for &(s, t) in &st {
        for i in 0..MAX {
            if !v[i] {
                continue;
            }
            let count = diff_count(i, s);
            if (t == 1 && count != 0) || (t == 2 && count != 1) || (t == 3 && count < 2) {
                v[i] = false;
            }
        }
    }

    let mut result = None;
    for i in 0..MAX {
        if v[i] {
            if result == None {
                result = Some(i);
            } else {
                println!("Can't Solve");
                return;
            }
        }
    }
    println!("{:0>4}", result.unwrap());
}
