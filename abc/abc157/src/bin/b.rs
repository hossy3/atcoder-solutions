use proconio::input;

fn main() {
    input! {
        a: [[usize; 3]; 3],
        n: usize,
        b: [usize; n],
    }
    let mut v = vec![false; 101];
    for b in b {
        v[b] = true;
    }

    let mut yes = false;
    for i in 0..3 {
        if v[a[i][0]] && v[a[i][1]] && v[a[i][2]] {
            yes = true;
        }
        if v[a[0][i]] && v[a[1][i]] && v[a[2][i]] {
            yes = true;
        }
    }
    if v[a[0][0]] && v[a[1][1]] && v[a[2][2]] {
        yes = true;
    }
    if v[a[2][0]] && v[a[1][1]] && v[a[0][2]] {
        yes = true;
    }

    println!("{}", if yes { "Yes" } else { "No" });
}
