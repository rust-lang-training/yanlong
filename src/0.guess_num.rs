use std::io;
use rand::Rng;
fn main () {
  loop {
    println!("请输入一个1~10之间的数字");
    let mut input_num = String::new();
    io::stdin().read_line(&mut input_num).expect("读取输入数字失败！");
    let input_num: u32 = input_num.trim().parse().expect("解析输入数字失败！");
    let target_num: u32 = rand::thread_rng().gen_range(0, 10); 

    println!("{} VS {}", input_num, target_num);

    if input_num > target_num {
      println!("YOU WIN!");
      break;
    } else if input_num == target_num {
      println!("DRAW!");
    } else {
      println!("YOU LOSE!");
    }
  }
  println!("GAME OVER");
}