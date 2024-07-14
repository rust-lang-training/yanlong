// 蒙特卡洛法计算派近似值
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;

const NUM_POINTS:usize = 1000000;
const NUM_THREADS:usize = 12;

fn main () {
  let result = Arc::new(Mutex::new(0));

  let mut handles = vec![];

  //创建多个线程计算
  for _ in 0..NUM_THREADS {
    let result_clone = result.clone();

    // 每个线程的计算逻辑
    let handle = thread::spawn(move || {
      let mut local_count = 0;
      let mut rng = rand::thread_rng();

      for _ in 0..NUM_POINTS {
        let x = rng.gen::<f64>();
        let y = rng.gen::<f64>();
        
        if x * x + y * y <= 1.0 {
          local_count += 1;
        }
      }

      let mut result = result_clone.lock().unwrap();
      *result += local_count;
    });

    handles.push(handle);
  }
  
  for handle in handles () {
    handle.join().unwrap();
  }

  let result = result.lock().unwrap();
  let pi_estimate = 4.0 * *result as f64 / (NUM_POINTS * NUM_THREADS) as f64;
  println!("Estimated value of PI : {}", pi_estimate);
}