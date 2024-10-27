// Collections are useful data structures in Rust
// store mutliple values
// data that these collections point to is stored on the heap.

use std::collections::HashMap;

fn even_filter(vec: &Vec<i32>) -> Vec<i32> {
  let mut new_vec = Vec::new();

  for val in vec {
    if val % 2 == 0 {
      new_vec.push(*val);
    }
  }

  return new_vec;
}

fn vectors() {
  let mut vec = Vec::new();
  vec.push(1);
  vec.push(2);
  vec.push(3);

  // let x = &vec[2];
  // vec[2] = 3; // throws error
  // println!("{}", x);

  let new_vec = even_filter(&vec);

  println!("new_vec: {:?}", new_vec);
  println!("vec: {:?}", vec);

  let vec_macro = vec![1, 2, 3];
  println!("vec_macro: {:?}", vec_macro);
}

fn hashmaps() {
  let mut hash_map = HashMap::new();

  // insert record in hashmap
  hash_map.insert(String::from("Sanchet"), 23);
  hash_map.insert(String::from("Joey"), 26);

  // get the record in hashmap
  match hash_map.get("Sanchet") {
    Some(val) => println!("Age is: {}", *val),
    None => println!("Key Sanchet does not exist.")
  }

  // remove record from hashmap
  match hash_map.remove("Joey") {
    Some(val) => println!("Key existed in the map, value is: {}", val),
    None => println!("Key did not exist in the map.")
  }
}

fn group_values_by_key(pairs: &Vec<(String, i32)>) -> HashMap<String, Vec<i32>> {
  let mut grouped_values: HashMap<String, Vec<i32>> = HashMap::new();

  for tup in pairs {
    let key = &tup.0;
    let val = tup.1;

    match grouped_values.get_mut(key) {
      Some(v) => v.push(val),
      None => match grouped_values.insert(key.to_string(), vec![val]) {
        Some(_v) => (),
        None => ()
      }
    }

    // Direct method
    // grouped_values.entry(key.clone()).or_insert(Vec::new()).push(val);
  }

  return grouped_values;
}

fn main() {
  vectors();
  hashmaps();

  let vec = vec![
    (String::from("Sanchet"), 12),
    (String::from("Arpita"), 9),
    (String::from("Eashan"), 16),
    (String::from("Sanchet"), -10),
    (String::from("Arpita"), 13),
    (String::from("Arpita"), 26),
    (String::from("Eashan"), -1)
  ];

  let grouped_values = group_values_by_key(&vec);

  println!("grouped values: {:?}", grouped_values);
}