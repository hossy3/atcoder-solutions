// let i0 = x % (m - 1);
// let mut sum = 0;
// let mut y = 1;
// for _ in 0..i0 {
//     sum = (sum + y) % m;
//     y = (y * a) % m;
// }
// if i0 < x {
//     let mut sum_loop = 0;
//     let mut y = 1;
//     for _ in 0..(m - 1) {
//         sum_loop = (sum_loop + y) % m;
//         y = (y * a) % m;
//     }
//     sum = sum + sum_loop * (x / (m - 1).max(1));
// }
