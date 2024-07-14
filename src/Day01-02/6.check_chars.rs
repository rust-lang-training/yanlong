use std::io;
fn main () {
  println!("请输入字符");
  let mut val = String::new();
  io::stdin().read_line(&mut val).expect("读取输入失败！");

  let mut results = [0u32; 26];

  for i in val.trim().to_lowercase().chars() {
    let code = i as u8;
    if 97 <= code && code <= 122 {
      results[(code - 97) as usize] = results[(code - 97) as usize] + 1; // a-z 对应为 0-25
    } 
  }

  for i in 0..26 {
    println!("{}.{} => {}", i, ((i as u8) + 97) as char, results[i]);
  }
}