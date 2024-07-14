mod foo; // 声明导入foo模块
// use crate::foo::Point;
fn main() {
  let a = foo::bar::foo_bar();
  println!("{}", a);

  let mut _x = foo::Point {
    x: 1.2,
    y: 2.1 
  };
  println!("{}", _x.move_pos());
}
