// use std::rc::Rc;
// use std::cell::RefCell;
use std::cmp::Ordering::*;

// struct Student {
//   active: bool,
//   username: String,
//   email: String,
//   sign_in_count: u64,
// }

// struct Point (f32, f32);

// struct Rectangle {
//   width: f32,
//   height: f32,
// }
// impl Rectangle {
//   fn square (size: f32) -> Self {
//     Self {
//       width: size,
//       height: size
//     }
//   }

//   fn perimeter(&self) -> f32 {
//     (self.width + self.height) * 2.0f32
//   }

//   fn scale(&mut self, width_scale:f32, height_scale:f32) {
//     self.width = self.width * width_scale;
//     self.height = self.height * height_scale;
//   }
// }

// enum Message {
//   Quit,
//   Move {x: i32, y: i32},
//   Write(String),
//   ChangeColor(i32, i32, i32)
// }

fn main () {
  //eg1
  // let s = String::from("hello");
  //   fn_other(s);
  //   let s = 1;
  //   println!("{}", s);
  // }

  // fn fn_other(val: String) {
  //   println!("{}", val);
  
  // eg2
  // let names = [
  //   String::from("John"),
  //   String::from("Tom"),
  //   String::from("Penny"),
  //   String::from("Sheldon"),
  // ];

  // for i in 0..4 {
  //   let s = names[i]; // 这⾏代码导致 names 拥有的值的所有权被转移
  //   // let s = &names[i]; //正确的
  //   println!("{}", s);
  // }

  // println!("names[0] = {}", names[0]);


  // eg3
  // let s = String::from("Hello world");
  // let bytes = s.into_bytes();
  //   // let bytes = s.clone().into_bytes();
  // println!("{:?}", s);

  // eg4
  // let x = 10;
  // let y = &x;

  // let mut m = 20;
  // let n = &mut m;
  // *n = 30;
  // println!("{}", y);
  // assert!(*y == 10);

  // eg5
  // let s1 = String::from("Hello world");
  // let len = calculate_length(&s1);

  // println!("{}, {}", s1, len);

  // eg6
  // let s1 = String::from("Hello world");
  // let s2 = &s1;
  // let s3 = &s1;
  // let s4 = &mut s1;

  // eg7 生命周期
  // longest();

  // eg8 
  // let mut s = String::from("Hello World");
  // let rs = &s;
  // // s.push_str(" I'm rust "); // push_str中创建了临时的可变引用
  // println!("{}", rs);

  // eg9
  // let s1 = String::from("hello world");
  // let rs1 = &s1;
  // let s2 = s1;
  // println!("{}", rs1);

  // eg10
  // let bytes = gen_string().as_bytes();
  // let binding = gen_string();
  // let bytes = binding.as_bytes();
  // println!("{:?}", bytes);

  //eg11
  // let s: Rc<String> = Rc::new(String::from("shirataki"));
  // let t: Rc<String> = s.clone();
  // let u: Rc<String> = s.clone();
  // println!("{}", Rc::strong_count(&s));
  // println!("{}", Rc::strong_count(&t));
  // println!("{}", Rc::strong_count(&u));

  //eg12
  // let s: Rc<String> = Rc::new(String::from("shiratki"));
  // println!("{}", Rc::strong_count(&s));
  // {

  // }

  // eg 13 栈上的字面量存放在堆上
  // let b = Box::new(5);
  // println!("{}", b)

  // eg14 实现内部可变性
  // let s = RefCell::new(String::from("hello world"));
  // append_string(&s);
  // println!("{}", s.borrow());

  // eg15 
  // let mut student = Student {
  //   active: true,
  //   username: "John Smith".to_string(),
  //   email: "j.smith@gmail.com".to_string(),
  //   sign_in_count: 0,
  // };

  // let Student {username, email, ..} = student;
  // println!("{}", username);

  // student.active = false;
  
  // eg 16

  // let p = Point(1.1, 2.3);
  // println!("{}, {}", p.0, p.1);

  // let Point(x, y) = p;
  // println!("{}, {}", x, y);

  // eg 17
  // let square = Rectangle::square(100.0f32);

  // println!("{:?}", square);
  

  // eg18 
  // let q = Message::Quit;
  // let m = Message::Move {x:100, y: 100};
  // println!("{:?}", q);

  // eg 19
  let s = describe_point(-1,2);
  println!("{}", s);
  

}
// fn calculate_length (s:&String) -> usize{
//   let ss = &s;
//   let sss = &&s;
//   s.len()
// }
// fn longest<'a> (x: &'a str, y: &'a str) -> &'a str{
//   if x.len() > y.len() {
//     x
//   } else {
//     y
//   }
// }
// fn gen_string() -> String {
//   String::from("hello world")
// }
// fn append_string (s: &RefCell<String>) {
//   let rs = s.borrow();
//   let mut ms = s.borrow_mut();
//   (*ms).push_str("I'm Rust");
//   println!("{}", rs);
// }

fn describe_point(x: i32, y: i32) -> &'static str {
  match (x.cmp(&0), y.cmp(&0)) {
    (Equal, Equal) => "at the origin",
    (_, Equal) => "at the x axis",
    (Equal, _) => "at the y axis",
    (Greater, Greater) => "in the first",
    (Less, Greater) => "at the second",
    _ => "at the origin"
  }
}