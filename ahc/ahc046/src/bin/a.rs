use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ij: [(usize, usize); m],
    }

    // 途中止まらない
    let (mut ci, mut cj) = ij[0];
    for &(i, j) in &ij[1..] {
        if ci > i {
            if i + 1 > ci - i {
                for _ in 0..(ci - i) {
                    println!("M U");
                }
            } else {
                println!("S U");
                for _ in 0..i {
                    println!("M D");
                }
            }
        } else if ci < i {
            if n - i > i - ci {
                for _ in 0..(i - ci) {
                    println!("M D");
                }
            } else {
                println!("S D");
                for _ in 0..(n - i - 1) {
                    println!("M U");
                }
            }
        }

        if cj > j {
            if n - j > cj - j {
                for _ in 0..(cj - j) {
                    println!("M L");
                }
            } else {
                println!("S R");
                for _ in 0..(n - j - 1) {
                    println!("M L");
                }
            }
        } else if cj < j {
            if j + 1 > j - cj {
                for _ in 0..(j - cj) {
                    println!("M R");
                }
            } else {
                println!("S L");
                for _ in 0..j {
                    println!("M R");
                }
            }
        }

        (ci, cj) = (i, j);
    }
}
