use std::{
  sync::{mpsc, Arc, Mutex},
  thread, vec,
};
fn is_prime_number(n: u32) -> bool {
  if n < 2 {
      return false;
  }
  let mut i = 2;
  while i * i <= n {
      if n % i == 0 {
          return false;
      }
      i += 1;
  }
  return true;
}

pub fn find_prime_number_mpsc(points: [(u32, u32); 5]) -> Vec<u32> {
  let (sender, receiver) = mpsc::channel::<Vec<u32>>();
  let result = Arc::new(Mutex::new(vec![]));
  let mut handles = vec![];
  let result_clone = result.clone();

  thread::spawn(move || {
      while let Ok(s) = receiver.recv() {
          let mut result = result_clone.lock().unwrap();
          result.append(&mut s.clone());
      }
  });

  for (point0, point1) in points.into_iter() {
      let tx: mpsc::Sender<Vec<u32>> = sender.clone();
      let handle = thread::spawn(move || {
          let mut current_result = vec![];
          for i in point0..=point1 {
              if is_prime_number(i) {
                  current_result.push(i);
              }
          }
          tx.send(current_result).unwrap();
      });
      handles.push(handle);
  }

  for handle in handles {
      handle.join().unwrap();
  }

  let result = result.lock().unwrap();
  result.clone()
}

pub fn find_prime_number(points: [(u32, u32); 5]) -> Vec<u32> {
  let mut handles = vec![];
  let result: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(vec![]));

  for (point0, point1) in points.into_iter() {
      let result_clone = result.clone();
      let handle = thread::spawn(move || {
          let mut current_result = vec![];
          for i in point0..=point1 {
              if self::is_prime_number(i) {
                  current_result.push(i);
              }
          }
          let mut result_clone = result_clone.lock().unwrap();
          result_clone.append(&mut current_result)
      });
      handles.push(handle);
  }

  for handle in handles {
      handle.join().unwrap();
  }

  let result = result.lock().unwrap();

  result.clone()
}