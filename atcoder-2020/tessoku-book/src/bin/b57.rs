use proconio::input;

// doubling

fn f(n: usize) -> usize {
    let mut m = n;
    let mut result = n;
    while m > 0 {
        result -= m % 10;
        m /= 10;
    }
    result
}

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut table = Vec::with_capacity(32);
    {
        let mut v = Vec::with_capacity(n + 1);
        for i in 0..=n {
            v.push(f(i));
        }
        table.push(v);
    }

    for i in 0..31 {
        let mut v = Vec::with_capacity(n + 1);
        for j in 0..=n {
            v.push(table[i][table[i][j]]);
        }
        table.push(v);
    }

    for i in 1..=n {
        let mut j = 0;
        let mut x = i; // value
        let mut y = k; // rest count
        while y > 0 {
            if y % 2 == 1 {
                x = table[j][x];
            }
            y /= 2;
            j += 1;
        }
        println!("{}", x);
    }
}
