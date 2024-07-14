// todo Trait: 抽象的定义共享行为；
// 类型的行为由该类型可调用的方法组成，不同的类型上面有相同的方法（共享了相同的行为）
// * trait: 抽象的定义共享行为，告诉编译器有哪些并且可以与其共享的功能；
// * 定义：把方法签名放在一起，来定义实现某种目的所必需的一组行为；
// * Trait bounds(约束): 指定了泛型类型参数实现了特定行为的类型,   
// todo 泛型提供多态，trait提供重载

use rust_homework::NewsArticle;
/**
 * 关键字：trait
 * 只有方法签名，没有具体实现；
 * trait可以有多个方法；
 * 实现该trait的类型必须提供具体的方法实现；
 */
use rust_homework::Summary; // 将trait引入到作用域才能使用
use rust_homework::Tweet;

fn main () {
  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false
  };
  println!("1 new tweet: {}", tweet.summarize());

  let article = NewsArticle {
    headline: String::from("勇士 VS 凯尔特人"),
    content: String::from("NBA 总决赛开始了"),
    author: String::from("shams"),
    location: String::from("Golden State"),
  };
  println!("1 new tweet: {}", article.summarize());
}