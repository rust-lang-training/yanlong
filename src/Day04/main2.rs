// use std::ops::Mul;
// struct Rectangle<T> {
//   width: T,
//   height: T,
// }
// impl <T : std::ops::Mul<Output = T> + Copy + std::ops::Add<Output = T>> Rectangle <T> {
//   fn area (&self) -> T {
//     self.width * self.height
//   }
//   fn perimeter (&self) -> T {
//     self.width + self.height
//   }
// }
fn main () {
  // let result = Rectangle.area();  
  // let x = 42;
  // let c1 = || println!("hello world");
  // let c2 = || println!("x = {}", x);
  // let c3 = || {
  //   let s = String::from("Hello World");
  //   println!("{}", s); 
  // };
  // c1();
  // c2();
  // c3();

  // let s = String::from("hello");
  // // let mut c1 = || s.push_str("Rust closure");
  // let c1 = move || {
  //   let tp = (s, 1);
  //   // s.push_str("Rust closure")
  //   println!("{:?}", tp);
  // };
  // c1();
  // c1();
  // println!("{}", s);

  let add_one = |x: i32| x + 1;
  let y = add_one(10);
  println!("{}", y);
}