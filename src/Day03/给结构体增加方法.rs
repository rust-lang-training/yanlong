// Todo 给结构体增加方法
// *结构体: 有关联关系的对象，往往将有联系的属性放在一起，组成一个个体，比如将长宽定义一个长方形；
#[derive(Debug)]
struct Rectangle {
  width: f32,
  height: f32
}

// 在 Rust 中，给结构体增加⽅法是使⽤ impl 块（Implementation），并且，⼀个结构体可以有多个 impl 块：
impl Rectangle {
  // *注入静态方法(关联函数)：不是在实例上调用的
  // 常用于构造器（创建这个struct的实例）; 例如：String::from()
  fn square(size: f32) -> Self {
    Self {
      width: size,
      height: size
    }
  }
  fn new (width: f32, height: f32) -> Self {
    Self {
      width,
      height
    }
  }

  // 注入实例方法
  fn perimeter (&self) -> f32 {
    (&self.width + &self.height) * 2.0f32
  }
  fn area (&self) -> f32 {
    self.width * self.height
  }
  fn scale (&mut self, width_scale: f32, height_scale: f32) {
    self.width = self.width * width_scale;
    self.height = self.height * height_scale;
  } 
}
fn main () {
  let square = Rectangle::square(100.0f32);
  println!("{:#?}", square);
  let mut rect = Rectangle::new(100.0f32, 100.0f32);
  println!("rect perimeter is: {}", rect.perimeter()); // 400
  println!("rect area is: {}", rect.area()); // 10000

  rect.scale(2.0, 2.0);
  println!("after scale 2: rect perimeter is: {}", rect.perimeter()); // 800
  println!("after scale 2: rect area is : {}", rect.area()); // 40000
}