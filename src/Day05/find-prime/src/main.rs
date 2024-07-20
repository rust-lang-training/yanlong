use find_prime::math::{find_prime_number, find_prime_number_mpsc};
// 找到100w以内的质数
const POINT: u32 = 1000000;
const COUNT: u32 = 5;
fn main() {
    let mut points = [(0, 0); 5];
    for i in 0..5 {
        points[i as usize] = (i * (POINT / COUNT as u32), (i + 1) * (POINT / COUNT as u32));
    }
    // 第一种方法多线程
    let result = find_prime_number(points);
    println!("多线程调用结果:{:?}", result);
    // 第二中mpsc方法
    let result = find_prime_number_mpsc(points);
    println!("mpsc调用结果:{:?}", result);
}