use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        a: [usize; n],
    }

    let mut v = vec![1usize];
    for a in a.windows(2) {
        if a[0].abs_diff(a[1]) >= d {
            let i = v.len() - 1;
            v[i] += 1;
        } else {
            v.push(1);
        }
    }

    eprintln!("{:?}", &v);

    let mut result = 0;
    for x in v {
        result += x * (x + 1) / 2;
    }
    println!("{result}");
}
