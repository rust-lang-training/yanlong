use std::fmt::Display;
use std::fmt::Debug;
pub trait Summary {
  fn summarize_author(&self) -> String; //方法的签名

  // 默认实现方法；可以调用
  fn summarize(&self) -> String{
    format!("(Read more from {} ...)", self.summarize_author())
  }
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

// 为NewsArticle类型实现Summary这个trait
impl Summary for NewsArticle {
  fn summarize_author(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}
pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

// 为Tweet类型实现Summary
impl Summary for Tweet {
  fn summarize_author(&self) -> String { // 将Summary中默认的summarize方法进行重写
    format!("{}, + {}", self.username, self.content)
  }
}

//*  将trait作为参数
// 概括的说就是item的参数类型实现了Summary这个trait，这样就可以调用Summary上的方法；
// 表示item是实现了Summary这个trait的某个类型
// + : 为类型实现多个trait

// 写法一：
pub fn notify1 (item: impl Summary + Display) {
  println!("Breaking news! {}", item.summarize());
}
// 写法二：trait bound 语法
pub fn notify2<T: Summary + Display, U: Clone + Debug> (item: T, _res: U) -> String {
  println!("Breaking news! {}", item.summarize());
}
// 写法三：多个类型参数的写法where 语句
pub fn notify3<T, U>(item: T, _res: U) -> String 
where
  T: Summary + Display,
  U: Clone + Debug
{
  println!("Breaking news! {}", item.summarize());
}


//*  将trait作为返回类型
// 让函数返回类型实现了某个trait：-> impl Trait