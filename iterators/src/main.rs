// Iterator pattern allows you to perform some task on a sequence of items in turn.
// iterators are lazy. just defining them would not do anything. Only when you use it.

use std::{collections::HashMap};

fn main() {
  let v1 = vec![1, 2, 3];

  // High level
  for v in v1 {
    println!("Got: {v}");
  }

  // Create immutable borrow iterator
  let v1 = vec![1, 2, 3];
  let imm_iter = v1.iter();
  for val in imm_iter {
    println!("Immutable iterator value: {}", *val);
  }

  // Create a mutable borrow iterator
  let mut v1 = vec![2, 3, 4];
  let mut_iter = v1.iter_mut();
  for v in mut_iter {
    *v = *v + 1;
    println!("New mutated value: {}", *v);
  }

  // Concise way
  let mut v1 = vec![1, 2, 3];
  let mut it = v1.iter_mut();
  while let Some(val) = it.next() {
    println!("Value: {}", val);
  }

  // create an iterator that owns the values
  // IntoIterator trait is used to convert collection to an iterator that takes
  // ownership of the collection
  let v1 = vec![1, 2, 3];
  let into_iter = v1.into_iter();
  for v in into_iter {
    println!("IntoIterator val: {}", v);
  }
  // println!("{:?}", v1); this would fail since the into_iter has assumed ownership of v1
  // and then consumed the data
  // The first way of iterating using the for loop (high level) uses this into_iter
  // iterator under the hood.

  // There are adapters which can be used with iterators:
  // 1. Consuming adapters: methods that use up/consume the iterator
  // 2. Iterator adapters: methods that generate a different iterator by changing
  // some aspect of the original iterators

  // Consuming adapters example
  let v1 = vec![1, 2, 3];
  let v1_iter = v1.iter();
  let sum: i32 = v1_iter.sum();
  assert_eq!(sum, 6);
  // now cannot use v1_iter anymore as v1_iter.sum() has consumed the iterator.

  // Iterator adapters example
  let v1 = vec![1, 2, 3];
  let v1_iter = v1.iter();

  // create a new iterator by mapping the elements to new values
  let map_iter = v1_iter.map(|x| 2 * x);
  for v in map_iter {
    println!("Mapped iterator: {v}");
  }
  // now cannot use v1_iter anymore since the ownership has moved to map_iter.


  // Example: write a logic to first filter all odd values, then double each value
  // and create a new vector
  let v1 = vec![1, 2, 3, 4, 5, 6];
  let v1_iter = v1.iter();
  let filter_iter = v1_iter.filter(|x| *x % 2 != 0);
  let map_iter = filter_iter.map(|x| x * 2);
  let new_v1: Vec<i32> = map_iter.collect();

  println!("Old v1: {:?}", v1);
  println!("New v1: {:?}", new_v1);

  // iterators on hashmaps
  let mut scores = HashMap::new();
  scores.insert("Alice", 20);
  scores.insert("Bob", 30);

  for (key, value) in scores.iter() {
    println!("{}: {}", key, value);
  }
}