use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut a = vec![];
    for _ in 0..m {
        input! {
            a0: [Usize1],
        }
        a.push(a0);
    }

    let mut stack = vec![];
    let mut v = vec![vec![]; n];
    for (i, a0) in a.iter().enumerate() {
        if let Some(&x) = a0.last() {
            v[x].push(i);
            if v[x].len() == 2 {
                stack.push(x);
            }
        }
    }

    while let Some(x) = stack.pop() {
        for i in 0..2 {
            let j = v[x][i];
            a[j].pop();
            if let Some(&x) = a[j].last() {
                v[x].push(j);
                if v[x].len() == 2 {
                    stack.push(x);
                }
            }
        }
    }

    let yes = v.iter().all(|x| x.len() == 2);
    println!("{}", if yes { "Yes" } else { "No" });
}
