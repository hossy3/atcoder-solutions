use proconio::{input, marker::Usize1};

// return: (値, 評価した長さ)
fn f(t: &[String]) -> (isize, usize) {
    match t[0].as_str() {
        "+" => {
            let (v0, l0) = f(&t[1..]);
            let (v1, l1) = f(&t[(1 + l0)..]);
            (v0 + v1, 1 + l0 + l1)
        }
        "-" => {
            let (v0, l0) = f(&t[1..]);
            let (v1, l1) = f(&t[(1 + l0)..]);
            (v0 - v1, 1 + l0 + l1)
        }
        "*" => {
            let (v0, l0) = f(&t[1..]);
            let (v1, l1) = f(&t[(1 + l0)..]);
            (v0 * v1, 1 + l0 + l1)
        }
        "/" => {
            let (v0, l0) = f(&t[1..]);
            let (v1, l1) = f(&t[(1 + l0)..]);
            (v0 / v1, 1 + l0 + l1)
        }
        _ => {
            let v = t[0].parse::<isize>().unwrap();
            (v, 1)
        }
    }
}

fn main() {
    input! {
        n: usize,
        t: [String; n],
        k: usize,
        p: [Usize1; k],
    }

    let mut t0 = t.clone();
    for p in p {
        t0[p] = match t[p].as_str() {
            "+" => String::from("-"),
            "-" => String::from("+"),
            "*" => String::from("/"),
            "/" => String::from("*"),
            _ => unreachable!(),
        };
    }

    println!("{}", f(&t).0);
    println!("{}", f(&t0).0);
}
