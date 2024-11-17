use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [usize; m],
        a: [usize; m],
    }

    if a.iter().sum::<usize>() != n {
        println!("-1");
        return;
    }

    let mut xa = vec![];
    for i in 0..m {
        xa.push((x[i], a[i]));
    }
    xa.sort();

    let mut cur = 1;
    let mut result = 0;
    for (x, a) in xa {
        if x > cur {
            println!("-1");
            return;
        }
        result += (cur - x) * a + a * (a - 1) / 2;
        cur += a;
    }
    println!("{result}");
}
