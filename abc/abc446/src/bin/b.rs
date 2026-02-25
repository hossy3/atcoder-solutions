use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut v = vec![true; m]; // ジュースがあるか
    'outer: for _ in 0..n {
        input! {
            l: usize,
            x: [Usize1; l],
        }
        for x in x {
            if v[x] {
                v[x] = false;
                println!("{}", x + 1);
                continue 'outer;
            }
        }
        println!("0");
    }
}
