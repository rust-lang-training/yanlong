use std::io;
fn main () {
  println!{"请输入五个整数，空格分隔"};

  let mut input_nums = String::new();
  io::stdin().read_line(&mut input_nums).expect("读取输入失效！");

  let mut numbers = [0u32; 5];

  let mut index = 0;
  for i in input_nums.trim().split(" ") {
    let i: u32 = i.trim().parse().expect("请输入有效的整数！");
    numbers[index] = i;
    index += 1;

    if index == 5 {
      break;
    }
    println!("this is {}", i);
  }

  let total = arr_add(&numbers[..]);
  println!("输入数字之和为 {}", total);
}

fn arr_add (items: &[u32]) -> u32 {
  let mut total = 0u32;
  for n in items {
    total += n;
  }
  total
}