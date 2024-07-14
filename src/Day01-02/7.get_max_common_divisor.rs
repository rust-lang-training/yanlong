use std::io;
fn main () {
  println!{"请输入两个整数，空格分隔"};

  let mut input_nums = String::new();
  io::stdin().read_line(&mut input_nums).expect("读取输入失效！");

  let mut numbers = [0u32; 2];
  let mut index = 0;
  for i in input_nums.trim().split(" ") {
    let i: u32 = i.trim().parse().expect("请输入有效的整数！");
    numbers[index] = i;
    index += 1;
  }
  
  let a = numbers[0];
  let b = numbers[1];

  println!{"第一个数：{}, 第二个数：{}", a, b};

  let result:u32 = get_max_common_divisor(a, b);
  println!{"{} 和 {} 的最大公约数为 {}", a, b, result};
}

fn get_max_common_divisor (a: u32, b: u32) -> u32 {
  let mut i = 1u32;
  let mut max = 1u32;
  while i <= a && i <= b {
    if a % i == 0 && b % i == 0 {
      max = i;
    }
    i += 1;
  }
  return max;
}
// 注意事项