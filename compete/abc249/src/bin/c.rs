use proconio::{input, marker::Chars};

fn rec(k: usize, s: &[Vec<char>], i: usize, count: &[usize]) -> usize {
    if i == s.len() {
        count.iter().filter(|&&x| x == k).count()
    } else {
        let mut count1 = count.to_vec();
        let s0 = &s[i];
        for i in 0..26 {
            if s0.contains(&((b'a' + i as u8) as char)) {
                count1[i] += 1;
            }
        }
        rec(k, s, i + 1, count).max(rec(k, s, i + 1, &count1))
    }
}

fn solve(k: usize, s: &[Vec<char>]) -> usize {
    let count = vec![0; 26];
    rec(k, s, 0, &count)
}

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [Chars; n],
    }
    let result = solve(k, &s);
    println!("{}", result);
}

// #[test]
// fn sample1() {
//     let s = vec![
//         vec!['a', 'b', 'i'],
//         vec!['a', 'e', 'f'],
//         vec!['b', 'c'],
//         vec!['a', 'c', 'g'],
//     ];
//     let result = solve(2, &s);
//     assert_eq!(result, 3);
// }
