use proconio::input;

fn prime_division(mut n: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let mut k = 2;
    while k * k <= n {
        let mut count = 0;
        while n % k == 0 {
            count += 1;
            n /= k;
        }
        if count > 0 {
            result.push((k, count));
        }
        k += if k == 2 { 1 } else { 2 };
    }
    if n > 1 {
        result.push((n, 1));
    }
    result
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut min = 0;
    let mut max = 0;
    for x in a {
        let pk = prime_division(x);

        let mut cell_num = 1;
        let mut op_num = 0;
        for &(p, k) in &pk {
            for _ in 0..k {
                op_num += cell_num;
                cell_num *= p;
            }
        }
        min += op_num;

        let mut cell_num = 1;
        let mut op_num = 0;
        for &(p, k) in pk.iter().rev() {
            for _ in 0..k {
                op_num += cell_num;
                cell_num *= p;
            }
        }
        max += op_num;
    }

    println!("{min} {max}");
}
