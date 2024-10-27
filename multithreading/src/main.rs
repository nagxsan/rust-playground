use std::thread;

fn main() {
  thread::spawn(|| {
    for i in 0..5 {
      println!("Spawned thread: {}", i);
    }
  });

  for i in 0..5 {
    println!("Main thread: {}", i);
  }


  let v = vec![1, 2, 3];
  // here without the `move` keyword, the thread cannot know if the vector `v` goes out of scope or not. Whenever we try to use a value from outside into the thread, we would need to move ownership of v into the closure.
  thread::spawn(move || {
    println!("v: {:?}", v);
  });
}