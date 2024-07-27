fn main () {
  // // let a = String::from("hello world");
  // // let a = [10, 11, 12];
  // let a = (10, 11);
  // let _b = a;
  // println!("{:?}", a);

  let mut x = String::from("hello world");
  let ptr = &mut x; // 创建一个可变引用
  ptr.push_str(" I'm Rust"); //.为自动解引用，并修改值
  println!("x = {:?}", x);
}