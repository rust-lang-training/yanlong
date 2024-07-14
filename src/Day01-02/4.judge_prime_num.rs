use std::io;
fn main () {
  println!("请输入需要判断的数");
  let mut num = String::new();
  io::stdin().read_line(&mut num).expect("读取输入项数失败！");
  let num: u64 = num.trim().parse().expect("输入值转换失败！");
  
  let sign: bool = judge_num(num);
  if sign {
    println!("YES!");
  } else {
    println!("NO!");
  }
}

fn judge_num (n: u64) -> bool {
  let mut i = 2u64;
  while i * i <= n {
    if n % i == 0 {
      return false;
    }
    i += 1;
  }
  return true;
}