use std::io;
fn main () {
  println!("请输入所求数列项数为：");
  let mut n = String::new();
  io::stdin().read_line(&mut n).expect("读取输入项数失败！");
  let n: u32 = n.trim().parse().expect("转换项数失败！");

  println!("使用循环方法输入 1");
  println!("使用迭代方法输入 2");
  let mut mode = String::new();
  io::stdin().read_line(&mut mode).expect("读取模式输入失败！");
  let mode = mode.trim();
  
  let mut result: u32 = 0u32;

  if mode == "1" {
    result = fibo_loop(n);
  }
  
  if mode == "2" {
    result = fibo_iter(n);
  }
  
  println!("斐波那契数列第 {} 项的值为 {}", n, result);
}

fn fibo_loop (val: u32) -> u32 {
  if val == 0 { 
    return 0;
  }

  if val == 1 || val == 2 {
    return 1;
  }

  let mut a = 1u32;
  let mut b = 1u32;
  
  for _ in 3..val {
    let m = a + b;
    a = b;
    b = m;
  }
  a + b
}

fn fibo_iter (val: u32) -> u32 {
  if val == 0 { 
    return 0;
  }

  if val == 1 || val == 2 {
    return 1;
  }
  
  return fibo_iter( val - 1 ) + fibo_iter (val - 2);
}