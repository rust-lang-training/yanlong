// todo 枚举
// *枚举：列举所有可能的值来定义一个类型；

// 定义一个Message类型
enum Message { 
  Quit,
  Move {x :i32, y: i32},
  Write(String),

}
#[derive(Debug)]
// ?枚举后面跟等号什么意思
enum HttpStatus {
  Ok = 200, 
  NotModified = 304,
  NotFound = 404,
}
struct Robot {
  name: String,
}

impl Robot {
  fn on_message (&mut self, msg: Message) {
    match msg {
      Message::Move {x, y} => println!("I'm going to Move: x => {}, y => {}", x, y),
      Message::Write(s) => println!("Hello {}", s),
      Message::Quit => println!("Bye bye ~"),
    }

  }
}

fn main () {
  // messages 是长度为3，类型是Message的数组
  let messages: [Message; 3] = [
    Message::Write("Hello you!".to_string()),
    Message::Move {x: 1, y: 2},
    Message::Quit,
  ];
   let https = HttpStatus::Ok;
   println!("{:#?}", https); // OK

  // let mut Robot = Robot {
  //   name: "Divad".to_string()
  // };
  // for msg in messages {
  //   Robot.on_message(msg);
  // }
}