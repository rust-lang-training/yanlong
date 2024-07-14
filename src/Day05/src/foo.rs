// 顶层就是foo
// mod foo {
  
// }
// mod bar;
// bar::foo_bar();
pub mod bar {
  pub fn foo_bar() -> String {
    String::from("Hello World")
  }
}

// pub enum TestEnum {
//   A,
//   B
// }

pub struct Point {
  pub x: f32,
  pub y: f32
}

impl Point {
  pub fn move_pos (&mut self) -> String {
    String::from("你好世界")
  }
}

