use std::io; // 标准库
fn main() {
  loop {
    println!("摄氏度和华氏度转化！");
    println!("摄氏度 -> 华氏度输入 0 ");
    println!("华氏度 -> 摄氏度输入 1 ");
  
    let mut mode = String::new();
    io::stdin().read_line(&mut mode).expect("读取输入失败！");
    let mode = mode.trim();
  
    if mode == "0" {
      println!("\n请输入摄氏度:");
      let mut degree = String::new();
      io::stdin().read_line(&mut degree).expect("读取输入失败！");
      let degree: f32 = degree.trim().parse().expect("请输入有效的数字！");
      let another_degree = degree * 1.8 + 32.0;
      println!("华氏度为 {}", another_degree);
    };
  
    if mode == "1" {
      println!("\n请输入华氏度:");
      let mut degree = String::new();
      io::stdin().read_line(&mut degree).expect("读取输入失败!");
      let degree: f32 = degree.trim().parse().expect("请输入有效的数字！");
      let another_degree = (degree - 32.0) / 1.8;  
      println!("摄氏度为 {}", another_degree);
    }
  }
}
