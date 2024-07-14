use std::io;
use std::io::Write;
use crate::Order;
use crate::Product;
use crate::Category;

// 打印主菜单
pub fn print_main_menu() {
  println!("\n---- 生鲜市场 ------");
  println!("0-退出 1-分类浏览商品 2-查询商品 3-下单商品 4-统计订单金额 5-支付订单");
}


// 获取输入值
pub fn get_stdin(tip: &str,) -> String {
  // 输入值不希望不换行
  print!("\n{tip}");
  // 默认情况下，标准输出是缓冲的，这意味着它不会立即将数据写入终端，而是先存储在缓冲区中，等到缓冲区满了或者遇到换行符时才实际写入。调用 flush() 会强制将缓冲区中的数据写入终端，即使缓冲区没有满
  io::stdout().flush().unwrap();
  let mut name = String::new();
  io::stdin().read_line(&mut name).unwrap();
  name.trim().to_string()
}

/**
* 菜单主函数 
*/
pub fn browse_products_menu(product_list: &Vec<Product>) {
  let category:u32 = get_stdin("0-all 1-fruit 2-vegetable 3-meat \n请选择想看的分类: ").parse().expect("请输入0-3的整数");
 
  let products_to_show = match category {
      // to_owned() 是一个更安全的替代方法，它不会导致所有权问题
      0 => product_list.to_owned(),
      // 各种报错： 1 => product_list.iter().filter(|p| p.category == Category::Fruit).collect()
      // filter_map 方法来创建一个新向量，包含与所选类别匹配的产品。没有使用 clone()，而是使用了 to_owned() 方法来创建一个新的所有权。避免了不必要的内存分配，并且代码更加简洁
      1 => product_list.iter().filter_map(|p| if p.category == Category::Fruit { Some(p.to_owned()) } else { None }).collect(),
      2 => product_list.iter().filter_map(|p| if p.category == Category::Vegetable { Some(p.to_owned()) } else { None }).collect(),
      3 => product_list.iter().filter_map(|p| if p.category == Category::Meat { Some(p.to_owned()) } else { None }).collect(),
      _ => {
          println!("请输入0-3的整数!");
          return;
      },
  };
  print_products(&products_to_show);
}

pub fn search_product_menu(product_list: &Vec<Product>) {
  let name = get_stdin("请输入商品名称: ");
  search_product(&name, &product_list);
}

pub fn order_product_menu(product_list: &mut Vec<Product>, order_list: &mut Vec<Order>) {
  let name = get_stdin("请输入选购的商品名称: ");
  let count: u32 = get_stdin("请输入选购的商品数量: ").parse().expect("请输入自然数");
  search_product(&name, &product_list);
  order_product(&name, count, product_list, order_list);

}

pub fn compute_order_menu(order_list:&Vec<Order>) {
  println!("目前订单如下：");
  print_orders(order_list);
  let sum = compute_order(&order_list);
  print_success(format!("当前订单总金额为：{}元!", sum).as_ref());
}

pub fn pay_order_menu(remain: &mut f32, order_list:&mut Vec<Order>) {
  let sum = compute_order(&order_list);
  println!("订单总金额为：{}元", sum);
  if sum == 0.0 {
      print_warn("您目前没有订单！");
      return;
  }
  match *remain > sum {
      true => {
          *remain -= sum;
          // 订单列表清空
          order_list.clear();
          print_success(format!("支付成功！您的当前余额为{}元", remain).as_ref());
      },
      false => {
          print_warn("余额不足，支付失败！");
      }
  }
}

/**
* 支持菜单的辅助函数
*/
pub fn print_products(product_list: &Vec<Product>) {
  print_table_header();
  for product in product_list {
      println!("{}", product);
  }
}

pub fn print_orders(order_list: &Vec<Order>) {
  // 打印表头
  println!(
      "{:<20} {:<20}  {:<20}  ",
      "Name", "Price", "Buy_Count",
  );
  println!("{}", "-".repeat(52));
  for o in order_list {
      println!("{}", o);
  }
}

pub fn search_product(name: &str, product_list: &Vec<Product>) {
  let product = product_list.iter().find(| p | {
      p.name == name
  } );
  
  match product {
      Some(p) => print_products(&vec![p.clone()]),
      _ => print_warn("没有该商品!"),
  }
}


pub fn order_product(name: &str, count: u32, product_list: &mut Vec<Product>, order_list: &mut Vec<Order>)  {
  // 这里需要更改库存，所以使用iter_mut
  let product = product_list.iter_mut().find(| p | {
      p.name == name
  } );
  
  match product {
      Some(p)  => {
          if p.stock >= count {
              p.stock -= count; 
              // 绿色显示
              println!("{}", format!("\x1b[32m下单成功!\x1b[0m"));
              println!("目前订单情况如下：");
              order_list.push(Order {
                  name: p.name.to_string(),
                  price: p.price,
                  buy_count: count
              });
              print_orders(order_list);
          } else {
              print_warn("库存不足，下单失败!");
          }
          
      },
      _ => print_warn("没有该商品，下单失败!"),
  }
  

}

pub fn compute_order(order_list:&Vec<Order>) -> f32 {
  order_list.iter().map(| o | o.price * (o.buy_count as f32)).sum()
}


// 打印表头
pub fn print_table_header() {
  println!(
      "{:<20} {:<20} {:<20} {:<20} ",
      "Name", "Category", "Stock", "Price",
  );
  println!("{}", "-".repeat(68));
}

// 红色警告
pub fn print_warn(tip: &str) {
  // \x1b 是 ESCape 序列的开始字符，[31m 将文本颜色设置为红色，而 \x1b[0m 将颜色重置回默认值
  println!("\x1b[31m{}\x1b[0m", tip);
}

// 绿色成功
pub fn print_success(tip: &str) {
  // \x1b 是 ESCape 序列的开始字符，[32m 将文本颜色设置为绿色，而 \x1b[0m 将颜色重置回默认值
  println!("\x1b[32m{}\x1b[0m", tip);
}