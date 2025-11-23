use proconio::input;

fn f(a: &[Vec<usize>]) -> bool {
    for i in 0..9 {
        if !g(&[
            a[i][0], a[i][1], a[i][2], a[i][3], a[i][4], a[i][5], a[i][6], a[i][7], a[i][8],
        ]) {
            return false;
        }
    }
    for i in 0..9 {
        if !g(&[
            a[0][i], a[1][i], a[2][i], a[3][i], a[4][i], a[5][i], a[6][i], a[7][i], a[8][i],
        ]) {
            return false;
        }
    }
    for i in 0..3 {
        for j in 0..3 {
            if !g(&[
                a[i * 3][j * 3],
                a[i * 3][j * 3 + 1],
                a[i * 3][j * 3 + 2],
                a[i * 3 + 1][j * 3],
                a[i * 3 + 1][j * 3 + 1],
                a[i * 3 + 1][j * 3 + 2],
                a[i * 3 + 2][j * 3],
                a[i * 3 + 2][j * 3 + 1],
                a[i * 3 + 2][j * 3 + 2],
            ]) {
                return false;
            }
        }
    }

    true
}

fn g(a: &[usize; 9]) -> bool {
    let mut a = a.clone();
    a.sort();
    a == [1, 2, 3, 4, 5, 6, 7, 8, 9]
}

fn main() {
    input! {
        a: [[usize; 9]; 9],
    }
    let yes = f(&a);
    println!("{}", if yes { "Yes" } else { "No" });
}
