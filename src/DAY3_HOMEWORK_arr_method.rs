use std::io;

fn main () {
  let mut a: Vec<i32> = vec![1, 7, 6, 4, 5, 8];

  println!("输入 1 ===> 返回其中最大的值");
  println!("输入 2 ===> 返回其中最小的值");
  println!("输入 3 ===> 返回其中最大值的引用");
  println!("输入 4 ===> 返回其中最小值的引用");
  println!("输入 5 ===> 返回元素的平均值");
  println!("输入 其他 ===> 返回其中每个元素都放大到原来的2倍");

  let mut mode = String::new();
  io::stdin().read_line(&mut mode).expect("读取输入数字失败！");
  let mode: &str = mode.trim();
  let value = &mut a[1..];
  println!("入参为 <===> {:?}", value);

  if mode == "1" || mode == "2" || mode == "5" {
    let result1 = handle_arr_method_one(value, mode);
    println!("最终结果为 <==> {}", result1);
    return;
  } else if mode == "3" || mode == "4" {
    let result2 = handle_arr_method_two(value, mode);
    println!("最终结果为 <==> {}", result2);
  } else {
    let result3 = handle_arr(value);
    println!("最终结果为 <==> {:?}", result3);
  }

}

fn handle_arr_method_one (val: &mut [i32], mode: &str) -> i32 {
  let _len = val.len();
  for i in 0.._len {
    for j in 0.._len - i - 1 {
      if val[j] > val[j + 1] {
        val.swap(j, j + 1);
      }
    }
  };

  let mut total = 0i32;
  for n in &mut *val {
    total += *n;
  }
  match mode {
    "1" => val[_len - 1],
    "2" => val[0],
    _ => total / 5
  }
  // if mode == "1" {
  //   return val[_len - 1];
  // } else if  mode == "2" {
  //  return  val[0];
  // } else {
  //   let mut total = 0i32;
  //   for n in val {
  //     total += *n;
  //   }
  //   total / 5
  // }
}
fn handle_arr_method_two<'a> (val: &'a mut [i32], mode: &'a str) -> &'a i32 {
  let _len = val.len();
  for i in 0.._len {
    for j in 0.._len - i - 1 {
      if val[j] > val[j + 1] {
        val.swap(j, j + 1);
      }
    }
  };
  
  match mode {
    "3" => &val[_len - 1],
    _ => &val[0]
  }

  // if mode == "3" {
  //   return &val[_len - 1];
  // } else {
  //  return  &val[0];
  // }
}

fn handle_arr (val: &mut [i32]) -> &[i32] {
  for i in 0..val.len() {
    val[i] = val[i] * 2;
  };
  val
}
