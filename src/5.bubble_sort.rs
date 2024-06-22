use rand::Rng;
fn main () {
  println!("随机生成 10 个数值");

  let mut numbers = [0u32; 10];

  for i in 0..10 {
    numbers[i] = rand::thread_rng().gen_range(0, 10);
  }
  println!("BEFORE: {:?}", numbers);
  
  let len = numbers.len();
  for i in 0..len {
    for j in 0..len - i - 1 {
      if numbers[j] > numbers[j + 1] {
        numbers.swap(j, j + 1);
      }
    }
  }

  println!("AFTER_SORT: {:?}", numbers);
}