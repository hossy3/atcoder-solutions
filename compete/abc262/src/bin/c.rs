use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    let mut b = Vec::<usize>::new();
    for i in 0..n {
        a[i] -= 1;
        if a[i] == i {
            b.push(i);
        }
    }
    let mut count = b.len() * (b.len() - 1) / 2;
    for i in 0..n {
        if a[i] > i && a[a[i]] == i {
            count += 1; 
        }
    }
    println!("{}", count);
}
