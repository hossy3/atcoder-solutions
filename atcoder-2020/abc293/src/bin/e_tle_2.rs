use proconio::input;

// 1 3 9 27 
// 1 3 2 6
// 1 3 1

fn main() {
    input! {
        a: usize,
        x: usize,
        m: usize,
    }

    let mut v = vec![0; 32];
    v[0] = 1;
    {
        let mut k = a;
        for i in 1..32 {
            for j in 0..i {
                v[i] += v[j];
            }
            v[i] = (v[i] * k) % m;
            k = (k * k) % m;
        }
    }

    println!("{:?}", v);

    let mut sum = 0;
    let mut k = 0;
    for i in (0..32).rev() {
        if (1usize << i) & x > 0 && k > 0 {
            sum = (sum + v[i] * k) % m;
        } 
        if (1usize << i) & x > 0 || k > 0 {
            sum = (sum + v[i]) % m;
            k += 1usize << i;
        }
    }

    println!("{}", sum);
}
